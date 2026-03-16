// XML属性値を enum として扱えるようにするマクロ
macro_rules! xml_enum {
    (enum $name:ident &str {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        xml_enum!(@define $name { $($variant = $val),* });

        impl ::core::str::FromStr for $name {
            type Err = ::core::convert::Infallible;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                ::core::result::Result::Ok(match s {
                    $($val => Self::$variant,)*
                    other => Self::Unknown(other.to_string()),
                })
            }
        }
    };

    (enum $name:ident $val_type:ty {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        xml_enum!(@define $name { $($variant = $val),* });

        impl $name {
            pub fn as_value(&self) -> ::core::option::Option<$val_type> {
                match self {
                    $(Self::$variant => ::core::option::Option::Some($val)),*,
                    Self::Unknown(_) => ::core::option::Option::None,
                }
            }
        }

        impl ::core::convert::From<$val_type> for $name {
            fn from(value: $val_type) -> Self {
                match value {
                    $($val => Self::$variant),*,
                    _ => Self::Unknown(value.to_string())
                }
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = ::core::convert::Infallible;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                ::core::result::Result::Ok(match <$val_type as ::core::str::FromStr>::from_str(s) {
                    $(::core::result::Result::Ok($val) => Self::$variant,)*
                    _ => Self::Unknown(s.to_owned())
                })
            }
        }
    };

    (@define $name:ident {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        pub enum $name {
            $($variant),*,
            Unknown(String),
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(Self::$variant => ::core::write!(f, "{}", $val),)*
                    Self::Unknown(v) => ::core::write!(f, "{v}"),
                }
            }
        }
    };
}

// Element をラップして属性と型の対応付けをした struct を作成
macro_rules! define_tag {
    ($name:ident { $($body:tt)* }) => {
        // エントリポイント
        #[derive(::core::fmt::Debug)]
        pub struct $name<E> {
            pub(crate) element: E,
        }

        impl<E> $name<E> {
            #[allow(dead_code)] // なぜかわからないけど warn が出る
            pub(crate) fn new(element: E) -> Self {
                Self { element }
            }
        }

        impl<E> $crate::helpers::FromElement<E> for $name<E> {
            fn from_element(element: E) -> Self {
                Self { element }
            }
        }

        define_tag!(@loop [] $name { $($body)* });
    };

    (@loop [$($acc:literal,)*] $name:ident {
        $attr_name:literal => $attr_ident:ident : enum $enum_name:ident &str {
            $($variant:ident = $val:expr),* $(,)?
        }
        $(, $($rest:tt)*)?
    }) => {
        // attr_type が enum (&str) の場合
        xml_enum! {
            enum $enum_name &str {
                $($variant = $val),*
            }
        }
        // 基本形に委譲
        define_tag!(@loop [$($acc,)*] $name { $attr_name => $attr_ident : $enum_name $(, $($rest)*)? });
    };

    (@loop [$($acc:literal,)*] $name:ident {
        $attr_name:literal => $attr_ident:ident : enum $enum_name:ident $val_type:ty {
            $($variant:ident = $val:expr),* $(,)?
        }
        $(, $($rest:tt)*)?
    }) => {
        // attr_type が enum の場合
        xml_enum! {
            enum $enum_name $val_type {
                $($variant = $val),*
            }
        }
        // 基本形に委譲
        define_tag!(@loop [$($acc,)*] $name { $attr_name => $attr_ident : $enum_name $(, $($rest)*)? });
    };

    (@loop [$($acc:literal,)*] $name:ident { $attr_name:literal => $attr_ident:ident : $attr_type:ty $(, $($rest:tt)*)? }) => {
        // 基本形 "attr_name" => attr_ident: attr_type
        define_tag!(@impl $name { $attr_name => $attr_ident : $attr_type });

        // 残りがあれば再帰的に処理
        define_tag!(@loop [$($acc,)* $attr_name,] $name { $($($rest)*)? });
    };

    (@loop [$($acc:literal,)*] $name:ident { $attr_name:literal : $($rest:tt)* }) => {
        // 省略形 "attr_name": attr_type
        ::paste::paste! {
            // 基本形に委譲
            define_tag!(@loop [$($acc,)*] $name { $attr_name => [<$attr_name>] : $($rest)* });
        }
    };

    // 終了条件
    (@loop [$($acc:literal,)*] $name:ident {}) => {
        impl<E> $name<E> {
            pub const ATTRIBUTES: [&'static str; [$($acc,)*].len()] = [$($acc,)*];
        }
    };

    (@impl $name:ident { $attr_name:literal => $attr_ident:ident : $attr_type:ty }) => {
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
    };
}

// Element をラップした型に対して、中身の単一子要素を取得するメソッドを定義
macro_rules! define_unique_children {
    ($name:ident { $($rest:tt)* }) => {
        // エントリポイント
        define_unique_children!(@loop [] $name { $($rest)* });
    };

    (@loop [$($acc:ident,)*] $name:ident { <$child_name:ident> => $child_ident:ident : $child_type:tt $(, $($rest:tt)*)? } ) => {
        // 基本形 <child_name> => child_ident: child_type
        define_unique_children!(@impl $name { <$child_name> => $child_ident : $child_type });

        // 残りがあれば再帰的に処理
        define_unique_children!(@loop [$($acc,)* $child_name,] $name { $($($rest)*)? });
    };

    (@loop [$($acc:ident,)*] $name:ident { <$child_name:ident> : $child_type:tt $(, $($rest:tt)*)? } ) => {
        // 省略形 <child_name>: child_type
        define_unique_children!(@loop [$($acc,)*] $name { <$child_name> => $child_name : $child_type $(, $($rest)*)? });
    };

    // 終了条件
    (@loop [$($acc:ident,)*] $name:ident {}) => {
        impl<E> $name<E> {
            pub const CHILDREN: [&str; [$(stringify!($acc),)*].len()] = [$(stringify!($acc),)*];
        }
    };

    (@impl $name:ident { <$child_name:ident> => $child_ident:ident : $child_type:tt } ) => {
        impl<E: $crate::domtree::HasChildren> $name<E> {
            pub fn $child_ident(&self) -> ::core::option::Option<$child_type<&$crate::domtree::Element>> {
                self.element.single_element_by_name(stringify!($child_name)).map(|(el, _)| $child_type::new(el))
            }
        }

        ::paste::paste! {
            impl<E: $crate::domtree::HasChildren + $crate::domtree::HasChildrenMut> $name<E> {
                pub fn [<$child_ident _mut>](&mut self) -> $child_type<&mut $crate::domtree::Element> {
                    let (el, _) = self.element.ensure_element(stringify!($child_name));
                    $child_type::new(el)
                }
            }
        }
    }
}

// Element をラップした型に対して、中身のリストを取得するメソッドを定義
macro_rules! define_lists {
    ($name:ident { $($rest:tt)* } ) => {
        // エントリポイント
        define_lists!(@loop [] $name { $($rest)* });
    };

    (@loop [$($acc:ident,)*] $name:ident { <$list_name:ident> => $list_ident:ident : [<$item_name:ident> : $item_type:tt] $(, $($rest:tt)*)? } ) => {
        // 基本形 <list_name> => list_ident: [<item_name>: item_type]
        define_lists!(@impl $name { <$list_name> => $list_ident : [<$item_name> : $item_type] });

        // 残りがあれば再帰的に処理
        define_lists!(@loop [$($acc,)* $list_name,] $name { $($($rest)*)? });
    };

    (@loop [$($acc:ident,)*] $name:ident { <$list_name:ident> : [<$item_name:ident> : $item_type:tt] $(, $($rest:tt)*)? } ) => {
        // 省略形 <list_name>: [<item_name>: item_type]
        define_lists!(@loop [$($acc,)*] $name { <$list_name> => $list_name : [<$item_name> : $item_type] $(, $($rest)*)? });
    };

    // 終了条件
    (@loop [$($acc:ident,)*] $name:ident {}) => {
        impl<E> $name<E> {
            pub const CHILD_LISTS: [&str; [$(stringify!($acc),)*].len()] = [$(stringify!($acc),)*];
        }
    };


    (@impl $name:ident { <$list_name:ident> => $list_ident:ident : [<$item_name:ident> : $item_type:tt] } ) => {
        impl<E: $crate::domtree::HasChildren> $name<E> {
            pub fn $list_ident(&self) -> ::core::option::Option<$crate::helpers::List<&$crate::domtree::Element, $item_type<&$crate::domtree::Element>>> {
                self.element
                    .single_element_by_name(stringify!($list_name))
                    .map(|(el, _)| $crate::helpers::List::new(el, stringify!($item_name)))
            }
        }

        ::paste::paste! {
            impl<E: $crate::domtree::HasChildren + $crate::domtree::HasChildrenMut> $name<E> {
                pub fn [<$list_ident _mut>](&mut self) -> $crate::helpers::List<&mut $crate::domtree::Element, $item_type<&mut $crate::domtree::Element>> {
                    let (el, _) = self.element
                        .ensure_element(stringify!($list_name));
                    $crate::helpers::List::new(el, stringify!($item_name))
                }
            }
        }
    }
}
