pub fn explain() {
    let some_string = Some(String::from("Hello, world!"));

    //prefix a variable with underscore so rust doesn't complain about it being unused
    if let Some(_)  = some_string {
        println!("String exists!");
    }
    
    //interesting semantics with just _ vs _varname

    //if you just use an underscore, no ownership will be transferred
    println!("{some_string:?}");
    //See, ownership wasn't transferred to the _ variable

    //but if you do name it...
    if let Some(_str) = some_string {
        println!("String exists... again!");
    }

    //Value is moved to the named variable and completely trashed
    // println!("{some_string:?}");
}