// XML属性値を enum として扱えるようにするマクロ
macro_rules! xml_enum {
    (enum $name:ident {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        pub enum $name {
            $($variant),*,
            Unknown(String),
        }

        impl std::str::FromStr for $name {
            type Err = std::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($val => Self::$variant,)*
                    other => Self::Unknown(other.to_string()),
                })
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $val),)*
                    Self::Unknown(v) => write!(f, "{v}"),
                }
            }
        }
    };
}

// Element をラップして属性と型の対応付けをした struct を作成
macro_rules! element_wrapper {
    ($tag_name:literal => $name:ident { $($body:tt)* }) => {
        #[derive(::core::fmt::Debug)]
        pub struct $name<E> {
            pub(crate) element: E,
        }

        impl<E> $crate::helpers::ListItem<E> for $name<E> {
            const NAME: &'static str = $tag_name;

            fn from_element(element: E) -> Self {
                Self { element }
            }
        }

        element_wrapper!(@impl $name { $($body)* });
    };

    (@impl $name:ident { $attr_name:literal => $attr_ident:ident : $attr_type:ty $(, $($rest:tt)*)? }) => {
        // 基本形 "attr_name" => attr_ident: attr_type
        impl<E: $crate::domtree::HasAttr> $name<E> {
            pub fn $attr_ident(&self) -> ::core::result::Result<$attr_type, $crate::domtree::error::AttrError> {
                self.element.attr($attr_name)
            }
        }

        ::paste::paste! {
            impl<E: $crate::domtree::HasAttrMut> $name<E> {
                pub fn [<set_ $attr_ident>](&mut self, value: $attr_type) {
                    self.element.set_attr($attr_name, value);
                }
            }
        }

        // 残りがあれば再帰的に処理
        $(element_wrapper!(@impl $name { $($rest)* });)?
    };

    (@impl $name:ident { $attr_name:literal : $attr_type:ty $(, $($rest:tt)*)? }) => {
        // 省略形 "attr_name": attr_type
        ::paste::paste! {
            // 文字列 $attr_name をそのまま識別子として扱う
            element_wrapper!(@impl $name { $attr_name => [<$attr_name>] : $attr_type });
        }

        // 残りがあれば再帰的に処理
        $(element_wrapper!(@impl $name { $($rest)* });)?
    };

    // 終了条件
    (@impl $name:ident {}) => {};
}

// Element をラップした型に対して、中身のリストを取得するメソッドを定義
macro_rules! impl_child_list {
    ($name:ident { $list_name:literal => $list_ident:ident : [$item_type:tt] $(, $($rest:tt)*)? } ) => {
        // 基本形 "list_name" => list_ident: item_type
        impl<E: $crate::domtree::HasChildren> $name<E> {
            pub fn $list_ident(&self) -> ::core::option::Option<$crate::helpers::List<&$crate::domtree::Element, $item_type<&$crate::domtree::Element>>> {
                self.element
                    .single_element_by_name($list_name)
                    .map(|(el, _)| $crate::helpers::List::new(el))
            }
        }

        ::paste::paste! {
            impl<E: $crate::domtree::HasChildren + $crate::domtree::HasChildrenMut> $name<E> {
                pub fn [<$list_ident _mut>](&mut self) -> $crate::helpers::List<&mut $crate::domtree::Element, $item_type<&mut $crate::domtree::Element>> {
                    let (el, _) = self.element
                        .ensure_element($list_name);
                    $crate::helpers::List::new(el)
                }
            }
        }

        // 残りがあれば再帰的に処理
        $(impl_child_list!($name { $($rest)* });)?
    };

    ($name:ident { $list_name:literal : [$item_type:tt] $(, $($rest:tt)*)? } ) => {
        // 省略形 "list_name": item_type
        ::paste::paste! {
            // 文字列 $list_name をそのまま識別子として扱う
            impl_child_list!($name { $list_name => [<$list_name>] : [$item_type] });
        }

        // 残りがあれば再帰的に処理
        $(impl_child_list!($name { $($rest)* });)?
    };

    // 終了条件
    ($name:ident {}) => {};
}
