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

        element_wrapper!(@impl $name, $($body)*);
    };

    (@impl $name:ident, $attr_name:literal => $attr_ident:ident : $attr_type:ty $(, $($rest:tt)*)?) => {
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
        $(element_wrapper!(@impl $name, $($rest)*);)?
    };

    (@impl $name:ident, $attr_name:literal : $attr_type:ty $(, $($rest:tt)*)?) => {
        // 省略形 "attr_name": attr_type
        ::paste::paste! {
            // 文字列 $key をそのまま識別子として扱う
            element_wrapper!(@impl $name, $attr_name => [<$attr_name>]: $attr_type);
        }

        // 残りがあれば再帰的に処理
        $(element_wrapper!(@impl $name, $($rest)*);)?
    };

    // 終了条件
    (@impl $name:ident,) => {};
}
