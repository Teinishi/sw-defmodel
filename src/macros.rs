macro_rules! get_first_stringify {
    ($first:tt $($rest:tt)*) => {
        stringify!($first)
    };
}

macro_rules! get_second_stringify {
    ($first:tt, $second:tt, $($rest:tt)*) => {
        stringify!($second)
    };
    ($first:tt $(,)?) => {
        stringify!($first)
    };
}

macro_rules! get_third_stringify {
    ($first:tt, $second:tt, $third:tt, $($rest:tt)*) => {
        stringify!($third)
    };
    ($first:tt, $second:tt $(,)?) => {
        stringify!($first)
    };
    ($first:tt $(,)?) => {
        stringify!($first)
    };
}

macro_rules! get_fourth_stringify {
    ($first:tt, $second:tt, $third:tt, $fourth:tt, $($rest:tt)*) => {
        stringify!($fourth)
    };
    ($first:tt, $second:tt, $third:tt $(,)?) => {
        stringify!($first)
    };
    ($first:tt, $second:tt $(,)?) => {
        stringify!($second)
    };
    ($first:tt $(,)?) => {
        stringify!($first)
    };
}

// Document をラップして便利メソッドを生やす
macro_rules! define_root {
    (
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        struct $name:ident {
            <$tag_name:ident> => $($root_type:tt)*
        }
    ) => {
        #[doc = $doc]
        $(#[$meta])*
        #[derive(Clone, Debug)]
        pub struct $name {
            tree: $crate::domtree::Document,
        }

        impl ComponentDefinition {
            pub fn from_xml_str(s: &str) -> ::core::result::Result<Self, $crate::domtree::ParseError> {
                Ok(Self {
                    tree: $crate::domtree::Document::from_xml_str(s)?,
                })
            }

            pub fn from_file<P: ::core::convert::AsRef<::std::path::Path>>(path: P) -> ::core::result::Result<Self, $crate::domtree::ParseError> {
                Ok(Self {
                    tree: $crate::domtree::Document::from_file(path)?,
                })
            }

            pub fn from_xml_reader<R: ::std::io::BufRead>(reader: &mut Reader<R>) -> ::core::result::Result<Self, $crate::domtree::ParseError> {
                Ok(Self {
                    tree: $crate::domtree::Document::from_xml_reader(reader)?,
                })
            }

            pub fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                self.tree.write(writer)
            }

            pub fn to_bytes(&self) -> ::std::io::Result<::std::vec::Vec<u8>> {
                self.tree.to_bytes()
            }

            pub fn $tag_name(&self) -> ::core::option::Option<$($root_type)*<&$crate::domtree::Element>> {
                $crate::domtree::HasChildren::single_element_by_name(&self.tree, stringify!($tag_name))
                    .map(|(el, _)| $($root_type)*::new(el))
            }

            ::paste::paste! {
                pub fn [<$tag_name _mut>](&mut self) -> $($root_type)*<&mut $crate::domtree::Element> {
                    let (el, _) = $crate::domtree::HasChildrenMut::ensure_element(&mut self.tree, stringify!($tag_name));
                    $($root_type)*::new(el)
                }
            }
        }
    };
}

// XML属性値を enum として扱えるようにするマクロ
macro_rules! xml_enum {
    ($(#[$meta:meta])* enum $name:ident &str {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        // 文字列型の場合
        xml_enum!(
            @define
            $(#[$meta])*
            #[doc = concat!(
                "| Value | Variant |\n",
                "|:------|:-------|\n",
                xml_enum!(@doc2 $name { $($variant = $val),* }),
                "```\n",
                "use ", module_path!(), "::", stringify!($name), ";\n",
                "\n",
                "// Parse from string\n",
                "let value = ", get_first_stringify!($($val),*), ".parse::<", stringify!($name), ">();\n",
                "assert_eq!(value, Ok(", stringify!($name), "::", get_first_stringify!($($variant),*), "));\n",
                "\n",
                "// Format to string\n",
                "assert_eq!(&format!(\"{}\", ", stringify!($name), "::", get_second_stringify!($($variant),*), "), ", get_second_stringify!($($val),*), ");\n",
                "```"
            )]
            $name { $($variant = $val),* }
        );

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

    ($(#[$meta:meta])* enum $name:ident $val_type:ty {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        // 文字列型以外の場合 (今のところ整数のみ想定)
        xml_enum!(
            @define
            $(#[$meta])*
            #[doc = concat!(
                "In the XML format, this value is stored as an integer.\n",
                "\n",
                "| `", stringify!($val_type), "` Value | Variant |\n",
                "|------:|:-------|\n",
                xml_enum!(@doc2 $name { $($variant = $val),* }),
                "```\n",
                "use ", module_path!(), "::", stringify!($name), ";\n",
                "\n",
                "// Parse from string\n",
                "let value = \"", get_first_stringify!($($val),*), "\".parse::<", stringify!($name), ">();\n",
                "assert_eq!(value, Ok(", stringify!($name), "::", get_first_stringify!($($variant),*), "));\n",
                "\n",
                "// Format to string\n",
                "assert_eq!(&format!(\"{}\", ", stringify!($name), "::", get_second_stringify!($($variant),*), "), \"", get_second_stringify!($($val),*), "\");\n",
                "\n",
                "// Convert from ", stringify!($val_type), "\n",
                "let value: ", stringify!($name), " = ", get_third_stringify!($($val),*), ".into();\n",
                "assert_eq!(value, ", stringify!($name), "::", get_third_stringify!($($variant),*), ");\n",
                "\n",
                "// Convert to ", stringify!($val_type), "\n",
                "assert_eq!(", stringify!($name), "::", get_fourth_stringify!($($variant),*), ".as_value(), Some(", get_fourth_stringify!($($val),*), "));\n",
                "```"
            )]
            $name { $($variant = $val),* }
        );

        impl $name {
            /// Returns the integer representation used in XML.
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

    (@doc2 $name:ident { $($variant:ident = $val:expr),* $(,)? }) => {
        concat!(
            $("| `", stringify!($val), "` | `", stringify!($variant), "` |\n", )*
            "\n",
            "Any other value is stored in [`", stringify!($name), "::Unknown`].\n",
            "\n",
            "# Examples\n",
            "\n",
        )
    };

    (@define $(#[$meta:meta])* $name:ident {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        $(#[$meta])*
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
    (#[doc = $doc:expr] $(#[$meta:meta])* struct $name:ident { $($body:tt)* }) => {
        // エントリポイント
        #[doc = $doc]
        #[doc = "\n\n"]
        #[doc = concat!(
            "To get access to attributes or children, typed getter/setters are provided for known ones.\n",
            "\n",
            "Attribute getters:\n",
            "- Return an enum value for attributes which is considered to be applicable to represent by an enum.\n",
            "    - Enums for attributes have `Unknown(String)` variant in preparation for future game update.\n",
            "- Return `Err(AttrError::NotFound)` if it does not exist in the XML.\n",
            "- Return `Err(Attr::ParseBoolError)`, `Err(Attr::ParseFloatError)` or `Err(Attr::ParseIntError)` if parsing the value failed.\n",
            "\n",
            "Child element getters:\n",
            "- Return the child element, or `None` if it does not exist.\n",
            "- If there are two or more in the XML, the last one will be returned.\n",
            "- Methods with `_mut` suffix in the name return mutable reference to the child element.\n",
            "    - It creates an element if it does not exist, as it will never fail. The return type is **not** `Option` nor `Result`.\n",
            "\n",
            "List getters:\n",
            "- Return the list of elements in the child element, or `None` if it does not exist.\n",
            "- Methods with `_mut` suffix in the name return list of mutable reference to the elements.\n",
            "    - It creates an element if it does not exist, as it will never fail. The return type is **not** `Option` nor `Result`.\n",
            // TODO: Examples
        )]
        $(#[$meta])*
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

        impl<'a> ::core::convert::From<&'a $crate::domtree::Element> for $name<&'a $crate::domtree::Element> {
            fn from(value: &'a $crate::domtree::Element) -> Self {
                Self { element: value }
            }
        }

        impl<'a> ::core::convert::From<&'a mut $crate::domtree::Element> for $name<&'a mut $crate::domtree::Element> {
            fn from(value: &'a mut $crate::domtree::Element) -> Self {
                Self { element: value }
            }
        }

        define_tag!(@loop [] $name { $($body)* });
    };

    (
        @loop [$($(#[$acc_meta:meta], )* $acc_name:literal, $acc_ident:ident, $acc_type:ty;)*]
        $name:ident {
            $(#[$attr_meta:meta])*
            $attr_name:literal => $attr_ident:ident : #[doc = $doc:expr] $(#[$enum_meta:meta])* enum $enum_name:ident &str {
                $($variant:ident = $val:expr),* $(,)?
            }
            $(, $($rest:tt)*)?
        }
    ) => {
        // attr_type が enum (&str) の場合
        xml_enum! {
            #[doc = $doc]
            $(#[$enum_meta])*
            enum $enum_name &str {
                $($variant = $val),*
            }
        }
        // 基本形に委譲
        define_tag!(
            @loop [$($(#[$acc_meta], )* $acc_name, $acc_ident, $acc_type;)*]
            $name { $(#[$attr_meta])* $attr_name => $attr_ident : $enum_name $(, $($rest)*)? }
        );
    };

    (
        @loop [$($(#[$acc_meta:meta], )* $acc_name:literal, $acc_ident:ident, $acc_type:ty;)*]
        $name:ident {
            $(#[$attr_meta:meta])*
            $attr_name:literal => $attr_ident:ident : #[doc = $doc:expr] $(#[$enum_meta:meta])* enum $enum_name:ident $val_type:ty {
                $($variant:ident = $val:expr),* $(,)?
            }
            $(, $($rest:tt)*)?
        }
    ) => {
        // attr_type が enum の場合
        xml_enum! {
            #[doc = $doc]
            $(#[$enum_meta])*
            enum $enum_name $val_type {
                $($variant = $val),*
            }
        }
        // 基本形に委譲
        define_tag!(
            @loop [$($(#[$acc_meta], )* $acc_name, $acc_ident, $acc_type;)*]
            $name { $(#[$attr_meta])* $attr_name => $attr_ident : $enum_name $(, $($rest)*)? }
        );
    };

    (
        @loop [$($(#[$acc_meta:meta], )* $acc_name:literal, $acc_ident:ident, $acc_type:ty;)*]
        $name:ident { $(#[$attr_meta:meta])* $attr_name:literal => $attr_ident:ident : $attr_type:ty $(, $($rest:tt)*)? }
    ) => {
        // 基本形 "attr_name" => attr_ident: attr_type
        // 残りがあれば再帰的に処理
        define_tag!(
            @loop [$($(#[$acc_meta], )* $acc_name, $acc_ident, $acc_type;)* $(#[$attr_meta], )* $attr_name, $attr_ident, $attr_type;]
            $name { $($($rest)*)? }
        );
    };

    (
        @loop [$($(#[$acc_meta:meta], )* $acc_name:literal, $acc_ident:ident, $acc_type:ty;)*]
        $name:ident { $(#[$attr_meta:meta])* $attr_name:literal : $($rest:tt)* }
    ) => {
        // 省略形 "attr_name": attr_type
        ::paste::paste! {
            // 基本形に委譲
            define_tag!(
                @loop [$($(#[$acc_meta], )* $acc_name, $acc_ident, $acc_type;)*]
                $name { $(#[$attr_meta])* $attr_name => [<$attr_name>] : $($rest)* }
            );
        }
    };

    // 終了条件 (属性なし)
    (
        @loop []
        $name:ident {}
    ) => {};

    // 終了条件
    (
        @loop [$($(#[$acc_meta:meta], )* $acc_name:literal, $acc_ident:ident, $acc_type:ty;)*]
        $name:ident {}
    ) => {
        impl<E> $name<E> {
            #[doc = concat!(
                "List of all known attributes.\n",
                "\n",
                "```\n",
                "# use ", module_path!(), "::", stringify!($name), ";\n",
                "# use sw_defmodel::domtree::Element;\n",
                "assert_eq!(\n",
                "    ", stringify!($name), "::<&Element>::ATTRIBUTES,\n",
                "    [", $(stringify!($acc_name), ", ", )* "]\n",
                ");\n",
                "```",
            )]
            pub const ATTRIBUTES: [&'static str; [$($acc_name,)*].len()] = [$($acc_name,)*];
        }

        impl<E: $crate::domtree::HasAttr> $name<E> {
            $(
                #[doc = concat!("Returns the value of `", $acc_name, "` attribute.")]
                $(#[$acc_meta])*
                pub fn $acc_ident(&self) -> ::core::result::Result<$acc_type, $crate::domtree::AttrError> {
                    self.element.attr($acc_name)
                }
            )*
        }

        ::paste::paste! {
            impl<E: $crate::domtree::HasAttrMut> $name<E> {
                $(
                    #[doc = concat!("Sets the value of `", $acc_name, "` attribute.")]
                    $(#[$acc_meta])*
                    pub fn [<set_ $acc_ident>](&mut self, value: $acc_type) {
                        self.element.set_attr($acc_name, value);
                    }
                )*
            }
        }

        // TODO: 属性・子要素の直接取得・操作
    };
}

// Element をラップした型に対して、中身の単一子要素を取得するメソッドを定義
macro_rules! define_unique_children {
    ($name:ident { $($rest:tt)* }) => {
        // エントリポイント
        define_unique_children!(@loop [] $name { $($rest)* });
    };

    (
        @loop [$($acc_name:ident, $acc_ident:ident, $acc_type:tt;)*]
        $name:ident { <$child_name:ident> => $child_ident:ident : $child_type:tt $(, $($rest:tt)*)? }
    ) => {
        // 基本形 <child_name> => child_ident: child_type
        // 残りがあれば再帰的に処理
        define_unique_children!(
            @loop [$($acc_name, $acc_ident, $acc_type;)* $child_name, $child_ident, $child_type;]
            $name { $($($rest)*)? }
        );
    };

    (
        @loop [$($acc_name:ident, $acc_ident:ident, $acc_type:tt;)*]
        $name:ident { <$child_name:ident> : $child_type:tt $(, $($rest:tt)*)? }
    ) => {
        // 省略形 <child_name>: child_type
        define_unique_children!(
            @loop [$($acc_name, $acc_ident, $acc_type;)*]
            $name { <$child_name> => $child_name : $child_type $(, $($rest)*)? }
        );
    };

    // 終了条件
    (
        @loop [$($acc_name:ident, $acc_ident:ident, $acc_type:tt;)*]
        $name:ident {}
    ) => {
        impl<E> $name<E> {
            #[doc = concat!(
                "List of all known unique child elements.\n",
                "\n",
                "```\n",
                "# use ", module_path!(), "::", stringify!($name), ";\n",
                "# use sw_defmodel::domtree::Element;\n",
                "assert_eq!(\n",
                "    ", stringify!($name), "::<&Element>::CHILDREN,\n",
                "    [", $("\"", stringify!($acc_name), "\", "),*, "]\n",
                ");\n",
                "```",
            )]
            pub const CHILDREN: [&str; [$(stringify!($acc_name),)*].len()] = [$(stringify!($acc_name),)*];
        }

        impl<E: $crate::domtree::HasChildren> $name<E> {
            $(
                #[doc = concat!("Returns the child `<", stringify!($acc_name), ">` element, or `None` if it does not exist.\n")]
                pub fn $acc_ident(&self) -> ::core::option::Option<$acc_type<&$crate::domtree::Element>> {
                    self.element.single_element_by_name(stringify!($acc_name)).map(|(el, _)| $acc_type::new(el))
                }
            )*
        }

        ::paste::paste! {
            impl<E: $crate::domtree::HasChildrenMut> $name<E> {
                $(
                    #[doc = concat!("Returns a mutable reference to the child `<", stringify!($acc_name), ">` element.",)]
                    pub fn [<$acc_ident _mut>](&mut self) -> $acc_type<&mut $crate::domtree::Element> {
                        let (el, _) = self.element.ensure_element(stringify!($acc_name));
                        $acc_type::new(el)
                    }
                )*
            }
        }
    };
}

// Element をラップした型に対して、中身のリストを取得するメソッドを定義
macro_rules! define_lists {
    ($name:ident { $($rest:tt)* } ) => {
        // エントリポイント
        define_lists!(@loop [] $name { $($rest)* });
    };

    (
        @loop [$($acc_name:ident, $acc_ident:ident, $acc_item_name:ident, $acc_item_type:tt;)*]
        $name:ident { <$list_name:ident> => $list_ident:ident : [<$item_name:ident> : $item_type:tt] $(, $($rest:tt)*)? }
    ) => {
        // 基本形 <list_name> => list_ident: [<item_name>: item_type]
        // 残りがあれば再帰的に処理
        define_lists!(
            @loop [$($acc_name, $acc_ident, $acc_item_name, $acc_item_type;)* $list_name, $list_ident, $item_name, $item_type;]
            $name { $($($rest)*)? }
        );
    };

    (
        @loop [$($acc_name:ident, $acc_ident:ident, $acc_item_name:ident, $acc_item_type:tt;)*]
        $name:ident { <$list_name:ident> : [<$item_name:ident> : $item_type:tt] $(, $($rest:tt)*)? }
    ) => {
        // 省略形 <list_name>: [<item_name>: item_type]
        define_lists!(
            @loop [$($acc_name, $acc_ident, $acc_item_name, $acc_item_type;)*]
            $name { <$list_name> => $list_name : [<$item_name> : $item_type] $(, $($rest)*)? }
        );
    };

    // 終了条件
    (
        @loop [$($acc_name:ident, $acc_ident:ident, $acc_item_name:ident, $acc_item_type:tt;)*]
        $name:ident {}
    ) => {
        impl<E> $name<E> {
            #[doc = concat!(
                "List of all known unique child list elements.\n",
                "\n",
                "```\n",
                "# use ", module_path!(), "::", stringify!($name), ";\n",
                "# use sw_defmodel::domtree::Element;\n",
                "assert_eq!(\n",
                "    ", stringify!($name), "::<&Element>::LISTS,\n",
                "    [", $("\"", stringify!($acc_name), "\", "),*, "]\n",
                ");\n",
                "```",
            )]
            pub const LISTS: [&str; [$(stringify!($acc_name),)*].len()] = [$(stringify!($acc_name),)*];
        }

        impl<E: $crate::domtree::HasChildren> $name<E> {
            $(
                #[doc = concat!("Returns the list of `<", stringify!($acc_item_name), ">` elements in the child `<", stringify!($acc_name), ">` element, or `None` if it does not exist.\n")]
                pub fn $acc_ident(&self) -> ::core::option::Option<$crate::helpers::List<&$crate::domtree::Element, $acc_item_type<&$crate::domtree::Element>>> {
                    self.element
                        .single_element_by_name(stringify!($acc_name))
                        .map(|(el, _)| $crate::helpers::List::new(el, stringify!($acc_item_name)))
                }
            )*
        }

        ::paste::paste! {
            impl<E: $crate::domtree::HasChildrenMut> $name<E> {
                $(
                    #[doc = concat!("Returns the list of mutable references to `<", stringify!($acc_item_name), ">` elements in the child `<", stringify!($acc_name), ">` element.\n")]
                    pub fn [<$acc_ident _mut>](&mut self) -> $crate::helpers::List<&mut $crate::domtree::Element, $acc_item_type<&mut $crate::domtree::Element>> {
                        let (el, _) = self.element
                            .ensure_element(stringify!($acc_name));
                        $crate::helpers::List::new(el, stringify!($acc_item_name))
                    }
                )*
            }
        }
    };
}
