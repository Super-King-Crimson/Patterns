pub fn explain() {
    //Let's say you want to have a match based on another variable
    let y = 5;
    let x = Some(10);

    match x {
        Some(1) => println!("Someone's a star!"),
        //THIS IS NOT Some(5)! This introduces a new shadowed variable named y, and will match any value in the Some
        Some(y) => println!("This should be 5, but the actual value is {y}"),
        _ => println!("idc lmoa"),
    }

    let x = Some(5);
    //Instead you have to use a match guard: an extra if condition in a match arm
    match x {
        Some(num) if num == y => println!("Now x and y are equal!"),
        _ => println!("Nobody cares lmoa"),
    }

    //match guards aren't patterns, and they're impossible to specify as patterns
    //They can capture outside variables and compare them

    //You can also use multiple match guards at once or apply match guards to multiple values
    let cond = true;
    let opt_val = Some(25);

    //Three guards: 
        //value exists
        //value is 1, 2, or 5
        //cond is true
    match opt_val {
        Some(1..=2 | 5) if cond => println!("Passed all guards"),
        Some(_) if cond => println!("Passed two guards"),
        None if !cond => println!("Passed no guards"),
        _ => println!("Passed one guard"),
    }
    //match guards apply to whole statement, so it's 
}