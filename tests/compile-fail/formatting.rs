#![feature(plugin)]
#![plugin(clippy)]

#![deny(clippy)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(if_same_then_else)]

fn foo() -> bool { true }

fn main() {
    // weird `else if` formatting:
    if foo() {
    } if foo() { //~ERROR this looks like an `else if` but the `else` is missing
    }

    let _ = {
        if foo() {
        } if foo() { //~ERROR this looks like an `else if` but the `else` is missing
        }
        else {
        }
    };

    if foo() {
    } else //~ERROR this is an `else if` but the formatting might hide it
    if foo() { // the span of the above error should continue here
    }

    if foo() {
    } //~ERROR this is an `else if` but the formatting might hide it
    else
    if foo() { // the span of the above error should continue here
    }

    // those are ok:
    if foo() {
    }
    if foo() {
    }

    if foo() {
    } else if foo() {
    }

    if foo() {
    }
    else if foo() {
    }

    if foo() {
    }

    else if

    foo() {}

    // weird op_eq formatting:
    let mut a = 42;
    a =- 35;
    //~^ ERROR this looks like you are trying to use `.. -= ..`, but you really are doing `.. = (- ..)`
    //~| NOTE to remove this lint, use either `-=` or `= -`
    a =* &191;
    //~^ ERROR this looks like you are trying to use `.. *= ..`, but you really are doing `.. = (* ..)`
    //~| NOTE to remove this lint, use either `*=` or `= *`

    let mut b = true;
    b =! false;
    //~^ ERROR this looks like you are trying to use `.. != ..`, but you really are doing `.. = (! ..)`
    //~| NOTE to remove this lint, use either `!=` or `= !`

    // those are ok:
    a = -35;
    a = *&191;
    b = !false;
}
