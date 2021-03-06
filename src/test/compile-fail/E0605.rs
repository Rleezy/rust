// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    let x = 0u8;
    x as Vec<u8>; //~ ERROR E0605
                  //~| NOTE an `as` expression can only be used to convert between primitive types

    let v = 0 as *const u8;
    v as &u8; //~ ERROR E0605
              //~| NOTE an `as` expression can only be used to convert between primitive types
}
