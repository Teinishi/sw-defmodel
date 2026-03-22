use std::{cmp::Ordering, collections::HashMap, hash::Hash};

#[derive(Debug)]
struct OrderGraph<T> {
    edges: HashMap<T, HashMap<T, usize>>,
}

impl<T> Default for OrderGraph<T> {
    fn default() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
}

impl<T: Hash + Eq + Clone> OrderGraph<T> {
    fn add_edge(&mut self, a: T, b: T, weight: usize) {
        *self.edges.entry(a).or_default().entry(b).or_insert(0) += weight;
    }

    fn add_sequence(&mut self, seq: &[T]) {
        for window in seq.windows(2) {
            self.add_edge(window[0].clone(), window[1].clone(), 1);
        }
    }

    fn merge_with(&mut self, other: Self) {
        for (a, weights) in other.edges {
            for (b, weight) in weights {
                self.add_edge(a.clone(), b, weight);
            }
        }
    }

    fn topo_sort<F>(&self, mut cmp: F) -> Vec<T>
    where
        F: FnMut(&[T], &T, &T) -> Ordering,
    {
        let mut indegree = HashMap::new(); // 入次数 (重みではない)
        let mut scores = HashMap::new(); // 出る辺の重み合計 - 入る辺の重み合計
        for (from, weights) in &self.edges {
            indegree.entry(from).or_insert(0);
            for (to, weight) in weights {
                *indegree.entry(to).or_insert(0) += 1_usize;
                *scores.entry(from).or_insert(0) += *weight as i32;
                *scores.entry(to).or_insert(0) -= *weight as i32;
            }
        }

        // 改変 Kahn 法
        let mut result: Vec<T> = Vec::new();
        let mut queue: Vec<&T> = Vec::new();

        while !indegree.is_empty() {
            let mut any_indegree_zero = false;
            for (node, deg) in &indegree {
                if *deg == 0 {
                    queue.push(*node);
                    any_indegree_zero = true;
                }
            }
            if !any_indegree_zero {
                // 入次数が 0 のノードがなければ、スコア最大のノードを対象にする
                let mut max_score: Option<(i32, Vec<&T>)> = None;
                for (node, score) in &scores {
                    if let Some((s, v)) = &mut max_score {
                        if score == s {
                            v.push(*node);
                        } else if score > s {
                            *s = *score;
                            v.clear();
                            v.push(*node);
                        }
                    } else {
                        max_score = Some((*score, vec![node]));
                    }
                }
                queue.extend(max_score.unwrap().1);
            }

            loop {
                let q_top = if queue.len() >= 2 {
                    // 一意性を確保するためにキューから優先度つきで取り出し
                    pop_best_with_cmp(&mut queue, |a, b| cmp(&result, a, b))
                } else {
                    queue.pop()
                };
                if q_top.is_none() {
                    break;
                }
                let node = q_top.unwrap();

                result.push(node.clone());
                indegree.remove(node);
                scores.remove(node);

                if let Some(weights) = self.edges.get(node) {
                    for (next, weight) in weights {
                        if let Some(deg) = indegree.get_mut(next) {
                            *deg -= 1;
                            if *deg == 0 {
                                queue.push(next);
                            }
                        }
                        if let Some(score) = scores.get_mut(next) {
                            *score += *weight as i32;
                        }
                    }
                }
            }
        }

        result
    }
}

fn pop_best_with_cmp<'a, T, F>(values: &'a mut Vec<&T>, mut cmp: F) -> Option<&'a T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    if values.is_empty() {
        return None;
    }

    let mut best_idx = 0;
    for i in 1..values.len() {
        if cmp(values[i], values[best_idx]) == Ordering::Less {
            best_idx = i;
        }
    }
    Some(values.swap_remove(best_idx))
}

#[derive(Debug, Default)]
pub struct OrderedMap<K, V> {
    map: HashMap<K, V>,
    order: OrderGraph<K>,
    sequence: Vec<K>,
}

#[allow(dead_code)]
impl<K: Hash + Eq + Clone, V> OrderedMap<K, V> {
    pub(super) fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    pub(super) fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.map.iter_mut()
    }

    pub(super) fn len(&self) -> usize {
        self.map.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(super) fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub(super) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub(super) fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub(super) fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.sequence.push(key.clone());
        self.map.insert(key, value)
    }

    pub(super) fn entry_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> &mut V {
        self.sequence.push(key.clone());
        self.map.entry(key).or_insert_with(f)
    }

    pub(super) fn add_sequence(&mut self, seq: &[K]) {
        self.order.add_sequence(seq);
    }

    pub(super) fn begin_sequence(&mut self) {
        debug_assert!(self.sequence.is_empty());
    }

    pub(super) fn end_sequence(&mut self) {
        self.order.add_sequence(&self.sequence);
        self.sequence.clear();
    }

    pub(super) fn merge_with(&mut self, other: Self, mut merge_value: impl FnMut(&mut V, V)) {
        for (k, v) in other.map {
            match self.map.get_mut(&k) {
                Some(existing) => merge_value(existing, v),
                None => {
                    self.map.insert(k, v);
                }
            }
        }

        self.order.merge_with(other.order);
    }
}

impl<K: AsRef<str> + Ord + Hash + Eq + Clone, V> OrderedMap<K, V> {
    pub(super) fn get_keys(&self) -> Vec<K> {
        self.order.topo_sort(prefix_priority)
    }

    pub(super) fn get_items(&self) -> Vec<(&K, &V)> {
        let order = self.get_keys();
        order
            .iter()
            .filter_map(|k| self.map.get_key_value(k))
            .collect()
    }
}

fn prefix_priority<K: AsRef<str> + Ord>(result: &[K], a: &K, b: &K) -> Ordering {
    let pa = result
        .last()
        .map(|prev| common_prefix_len(prev.as_ref(), a.as_ref()))
        .unwrap_or(0);

    let pb = result
        .last()
        .map(|prev| common_prefix_len(prev.as_ref(), b.as_ref()))
        .unwrap_or(0);

    pb.cmp(&pa).then_with(|| a.cmp(b))
}

fn common_prefix_len(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_order() {
        let mut m = OrderedMap::default();

        m.begin_sequence();
        m.insert("one", ());
        m.insert("two", ());
        m.insert("three", ());
        m.insert("four", ());
        m.end_sequence();

        assert_eq!(m.get_keys(), vec!["one", "two", "three", "four"]);
    }

    #[test]
    fn merge_partial_order_2() {
        let mut m = OrderedMap::default();

        m.begin_sequence();
        m.insert("one", ());
        m.insert("three", ());
        m.insert("four", ());
        m.end_sequence();

        m.begin_sequence();
        m.insert("one", ());
        m.insert("two", ());
        m.insert("three", ());
        m.insert("four", ());
        m.end_sequence();

        assert_eq!(m.get_keys(), vec!["one", "two", "three", "four"]);
    }

    #[test]
    fn merge_partial_order_3() {
        let mut m = OrderedMap::default();

        m.begin_sequence();
        m.insert("one", ());
        m.insert("three", ());
        m.end_sequence();

        m.begin_sequence();
        m.insert("two", ());
        m.insert("three", ());
        m.insert("four", ());
        m.end_sequence();

        m.begin_sequence();
        m.insert("one", ());
        m.insert("two", ());
        m.insert("four", ());
        m.end_sequence();

        assert_eq!(m.get_keys(), vec!["one", "two", "three", "four"]);
    }

    #[test]
    fn order_frequency() {
        let mut m = OrderedMap::default();

        for _ in 0..2 {
            m.begin_sequence();
            m.insert("one", ());
            m.insert("four", ());
            m.insert("two", ());
            m.insert("three", ());
            m.end_sequence();
        }

        for _ in 0..3 {
            m.begin_sequence();
            m.insert("one", ());
            m.insert("two", ());
            m.insert("three", ());
            m.insert("four", ());
            m.end_sequence();
        }

        m.order.topo_sort(prefix_priority);

        assert_eq!(m.get_keys(), vec!["one", "two", "three", "four"]);
    }

    #[test]
    fn order_priority() {
        let mut m = OrderedMap::default();

        m.begin_sequence();
        m.insert("three", ());
        m.insert("three_0", ());
        m.insert("three_1", ());
        m.insert("five", ());
        m.end_sequence();

        m.begin_sequence();
        m.insert("one", ());
        m.insert("two", ());
        m.insert("three", ());
        m.insert("four", ());
        m.insert("five", ());
        m.end_sequence();

        assert_eq!(
            m.order.topo_sort(|_r, a, b| Ord::cmp(a, b)),
            vec!["one", "two", "three", "four", "three_0", "three_1", "five"]
        );

        assert_eq!(
            m.get_keys(),
            vec!["one", "two", "three", "three_0", "three_1", "four", "five"]
        );
    }
}
