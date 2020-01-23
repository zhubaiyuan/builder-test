#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use builder::Builder;

struct X {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for X {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            X {  } => {
                let mut debug_trait_builder = f.debug_struct("X");
                debug_trait_builder.finish()
            }
        }
    }
}

struct Item<T, U> where T: Default {
    a: u32,
    b: Option<&'static str>,
    c: String,
    #[builder(required)]
    d: X,
    e: T,
    #[builder(required)]
    f: U,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::core::fmt::Debug, U: ::core::fmt::Debug> ::core::fmt::Debug for
 Item<T, U> where T: Default {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Item {
            a: ref __self_0_0,
            b: ref __self_0_1,
            c: ref __self_0_2,
            d: ref __self_0_3,
            e: ref __self_0_4,
            f: ref __self_0_5 } => {
                let mut debug_trait_builder = f.debug_struct("Item");
                let _ = debug_trait_builder.field("a", &&(*__self_0_0));
                let _ = debug_trait_builder.field("b", &&(*__self_0_1));
                let _ = debug_trait_builder.field("c", &&(*__self_0_2));
                let _ = debug_trait_builder.field("d", &&(*__self_0_3));
                let _ = debug_trait_builder.field("e", &&(*__self_0_4));
                let _ = debug_trait_builder.field("f", &&(*__self_0_5));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <T, U> Item<T, U> where T: Default {
    fn builder() -> ItemBuilder<T, U> { ItemBuilder::new() }
}
impl <T, U> Default for ItemBuilder<T, U> where T: Default {
    fn default() -> Self {
        ItemBuilder{a: None, b: None, c: None, d: None, e: None, f: None,}
    }
}
struct ItemBuilder<T, U> where T: Default {
    a: Option<u32>,
    b: Option<Option<&'static str>>,
    c: Option<String>,
    d: Option<X>,
    e: Option<T>,
    f: Option<U>,
}
impl <T, U> ItemBuilder<T, U> where T: Default {
    fn new() -> Self { Default::default() }
    fn a<__Builder_T: Into<u32>>(mut self, val: __Builder_T) -> Self {
        self.a = Some(val.into());
        self
    }
    fn b<__Builder_T: Into<Option<&'static str>>>(mut self, val: __Builder_T)
     -> Self {
        self.b = Some(val.into());
        self
    }
    fn c<__Builder_T: Into<String>>(mut self, val: __Builder_T) -> Self {
        self.c = Some(val.into());
        self
    }
    fn d<__Builder_T: Into<X>>(mut self, val: __Builder_T) -> Self {
        self.d = Some(val.into());
        self
    }
    fn e<__Builder_T: Into<T>>(mut self, val: __Builder_T) -> Self {
        self.e = Some(val.into());
        self
    }
    fn f<__Builder_T: Into<U>>(mut self, val: __Builder_T) -> Self {
        self.f = Some(val.into());
        self
    }
    fn build(self) -> Item<T, U> {
        Item{a: self.a.unwrap_or_else(Default::default),
             b: self.b.unwrap_or_else(Default::default),
             c: self.c.unwrap_or_else(Default::default),
             d: self.d.unwrap(),
             e: self.e.unwrap_or_else(Default::default),
             f: self.f.unwrap(),}
    }
}

fn main() {
    let item: Item<i32, &str> =
        Item::builder().a(42u32).b("hello").c("boom".to_owned()).d(X{}).e(42i32).f("hello").build();

    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(&["",
                                                                     "\n"],
                                                                   &match (&item,)
                                                                        {
                                                                        (arg0,)
                                                                        =>
                                                                        [::core::fmt::ArgumentV1::new(arg0,
                                                                                                      ::core::fmt::Debug::fmt)],
                                                                    },
                                                                   &[::core::fmt::rt::v1::Argument{position:
                                                                                                       0usize,
                                                                                                   format:
                                                                                                       ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                           ' ',
                                                                                                                                       align:
                                                                                                                                           ::core::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                       flags:
                                                                                                                                           4u32,
                                                                                                                                       precision:
                                                                                                                                           ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                       width:
                                                                                                                                           ::core::fmt::rt::v1::Count::Implied,},}]));
    };
    let item2 = Item::<u32, u64>::builder().b(None).d(X{}).f(99u64).build();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(&["",
                                                                     "\n"],
                                                                   &match (&item2,)
                                                                        {
                                                                        (arg0,)
                                                                        =>
                                                                        [::core::fmt::ArgumentV1::new(arg0,
                                                                                                      ::core::fmt::Debug::fmt)],
                                                                    },
                                                                   &[::core::fmt::rt::v1::Argument{position:
                                                                                                       0usize,
                                                                                                   format:
                                                                                                       ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                           ' ',
                                                                                                                                       align:
                                                                                                                                           ::core::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                       flags:
                                                                                                                                           4u32,
                                                                                                                                       precision:
                                                                                                                                           ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                       width:
                                                                                                                                           ::core::fmt::rt::v1::Count::Implied,},}]));
    };
}
