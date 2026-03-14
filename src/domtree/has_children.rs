use super::{Element, Node, utils::trailing_whitespaces};

pub(crate) trait HasChildren {
    fn children(&self) -> &Vec<Node>;
    fn children_mut(&mut self) -> &mut Vec<Node>;

    fn get_child(&self, index: usize) -> Option<&Node> {
        self.children().get(index)
    }
    fn get_child_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.children_mut().get_mut(index)
    }
    fn remove_child(&mut self, index: usize) -> Node {
        self.children_mut().remove(index)
    }
    fn insert_child(&mut self, index: usize, node: Node) {
        self.children_mut().insert(index, node);
    }

    fn elements(&self) -> impl Iterator<Item = (&Element, usize)> {
        self.children().iter().enumerate().filter_map(|(i, node)| {
            if let Node::Element(el) = &node {
                Some((el, i))
            } else {
                None
            }
        })
    }
    fn elements_mut(&mut self) -> impl Iterator<Item = (&mut Element, usize)> {
        self.children_mut()
            .iter_mut()
            .enumerate()
            .filter_map(|(i, node)| {
                if let Node::Element(el) = node {
                    Some((el, i))
                } else {
                    None
                }
            })
    }

    fn elements_by_name<K: AsRef<[u8]>>(&self, name: K) -> impl Iterator<Item = (&Element, usize)> {
        self.elements()
            .filter(move |(el, _)| el.name == name.as_ref())
    }
    fn elements_by_name_mut<K: AsRef<[u8]>>(
        &mut self,
        name: K,
    ) -> impl Iterator<Item = (&mut Element, usize)> {
        self.elements_mut()
            .filter(move |(el, _)| el.name == name.as_ref())
    }

    fn single_element_by_name<K: AsRef<[u8]>>(&self, name: K) -> Option<(&Element, usize)> {
        self.elements().find(|(el, _)| el.name == name.as_ref())
    }
    fn single_element_by_name_mut<K: AsRef<[u8]>>(
        &mut self,
        name: K,
    ) -> Option<(&mut Element, usize)> {
        self.elements_mut().find(|(el, _)| el.name == name.as_ref())
    }

    fn remove_element(&mut self, index: usize) -> Node {
        let removed = self.remove_child(index);
        if index >= 1
            && let Some(Node::Text(text)) = self.get_child_mut(index - 1)
        {
            // 直前にある空白は削除する
            let i = text
                .iter()
                .rev()
                .position(|c| !c.is_ascii_whitespace())
                .map(|i| text.len() - 1 - i)
                .unwrap_or(0);
            if i == 0 {
                self.remove_child(index - 1);
            } else {
                text.truncate(i);
            }
        }
        removed
    }
    fn insert_element_before(&mut self, index: usize, element: Element) {
        // index で指定した位置の要素を後ろに押しのけて新しい要素を挿入する
        if index >= 1
            && let Some(ws) = self
                .get_child(index - 1)
                .and_then(node_trailing_whitespaces)
        {
            // 直前にある空白 (インデント等) はコピーする
            self.children_mut()
                .splice(index..index, [Node::Element(element), Node::Text(ws)]);
        } else {
            self.insert_child(index, Node::Element(element));
        }
    }
    fn insert_element_after(&mut self, index: usize, element: Element) {
        // index で指定した位置の要素の後ろに新しい要素を挿入する
        if index >= 1
            && let Some(ws) = self
                .get_child(index - 1)
                .and_then(node_trailing_whitespaces)
        {
            // 直前にある空白 (インデント等) はコピーする
            self.children_mut().splice(
                index + 1..index + 1,
                [Node::Text(ws), Node::Element(element)],
            );
        } else {
            self.insert_child(index + 1, Node::Element(element));
        }
    }
}

fn node_trailing_whitespaces(leading_node: &Node) -> Option<Vec<u8>> {
    if let Node::Text(text) = leading_node {
        let s = trailing_whitespaces(text);
        if !s.is_empty() {
            return Some(s.to_vec());
        }
    }
    None
}
