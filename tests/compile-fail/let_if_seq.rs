#![feature(plugin)]
#![plugin(clippy)]

#![allow(unused_variables, unused_assignments, similar_names, blacklisted_name)]
#![deny(useless_let_if_seq)]

fn f() -> bool { true }

fn main() {
    let mut foo = 0;
    //~^ ERROR `let foo;`/`if .. { foo =  ..; }` sequence detected
    //~| HELP more idiomatic
    //~| SUGGESTION let mut foo = if f() { 42 } else { 0 };
    if f() {
        foo = 42;
    }

    let mut bar = 0;
    //~^ ERROR `let foo;`/`if .. { foo =  ..; }` sequence detected
    //~| HELP more idiomatic
    //~| SUGGESTION let mut bar = if f() { ..; 42 } else { ..; 0 };
    if f() {
        f();
        bar = 42;
    }
    else {
        f();
    }

    let quz;
    //~^ ERROR `let foo;`/`if .. { foo =  ..; }` sequence detected
    //~| HELP more idiomatic
    //~| SUGGESTION let mut quz = if f() { ..; 42 } else { ..; 0 };

    if f() {
        quz = 42;
    } else {
        quz = 0;
    }

    // baz needs to be mut
    let mut baz = 0;
    //~^ ERROR `let foo;`/`if .. { foo =  ..; }` sequence detected
    //~| HELP more idiomatic
    //~| SUGGESTION let mut baz = if f() { ..; 42 } else { ..; 0 };
    if f() {
        baz = 42;
    }

    baz = 1337;
}
