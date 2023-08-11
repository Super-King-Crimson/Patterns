pub fn explain() {
    println!("Refutability is just whether or not a pattern can fail");

    //A match statement or let assignment is IRREFUTABLE
    //because the compiler enforces that you MUST catch every case, every pattern a value can hold
    let optional_enum = Some(IntStr::Int(10)); //impossible to fail a match: always will be assigned Option<IntStr<'_>>

    match optional_enum {
        Some(Int(1)) => (),
        Some(Str("one")) => (),
        //if you remove any of the bottom 3 catchalls the compiler will crap its pantsu
        Some(Int(_)) => (),
        Some(Str(_)) => (),
        None => (),
    }

    //On the other hand, if let can also accept REFUTABLE patterns
    if let Some(_) = Option::<u32>::None {
        println!("We have a value");
    }
    //this will not match when the matching value is None, so it can be refuted

    //if you put a refutable pattern where an irrefutable one is expected compiler will yell
    let _a = if let Int(_val) = IntStr::Int(100) {
        _val
    } else {
        90
    };
    // - refutable pattern in local binding
    //we cannot cover EVERY pattern ANY value could take on the left hand of this assignment, so Rust complains
    //If Rust allowed this to compile it wouldn't know what to assign val if the pattern failed to match
        //We can fix this by putting an if in front of it then assigning that expression to a let variable

    //If we pass an irrefutable binding to a place where rust can take a refutable one, we get warned
    if let _x = 5 {
        println!("This cannot fail!");
    } else {
        println!("This is unreachable!");
    }
}

use IntStr::*;
pub enum IntStr<'a> {
    Int(i32),
    Str(&'a str)
}