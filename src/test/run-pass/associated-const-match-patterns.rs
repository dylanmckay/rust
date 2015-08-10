// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(associated_consts)]
#![feature(const_fn)]

struct Foo;

enum Bar {
    Var1,
    Var2,
}

// Use inherent and trait impls to test UFCS syntax.
impl Foo {
    const fn new() -> Bar { Bar::Var1 }

    const MYBAR: Bar = Bar::Var2;
    const MYFOO: Bar = Foo::new();
}

trait HasBar {
    const THEBAR: Bar;
}

impl HasBar for Foo {
    const THEBAR: Bar = Bar::Var1;
}

fn main() {
    // Inherent impl
    assert!(match Bar::Var2 {
        Foo::MYBAR => true,
        _ => false,
    });
    assert!(match Bar::Var2 {
        <Foo>::MYBAR => true,
        _ => false,
    });
    assert!(match Bar::Var1 {
        Foo::MYFOO => true,
        _ => false,
    });
    // Trait impl
    assert!(match Bar::Var1 {
        Foo::THEBAR => true,
        _ => false,
    });
    assert!(match Bar::Var1 {
        <Foo>::THEBAR => true,
        _ => false,
    });
    assert!(match Bar::Var1 {
        <Foo as HasBar>::THEBAR => true,
        _ => false,
    });
}
