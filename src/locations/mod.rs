pub fn explain() {
    println!("Patterns show up in a variety in locations");

    //For race_placeample...
    match_arms();
    if_else_while_let();
    for_loops();
    let_assignment();
    fn_params(18, "this function is stupid", &vec![String::from("Why'd"), String::from("I"), String::from("Write this"), String::from("Crap")]);
}

fn match_arms() {
    //This is the default use location of patterns
    let race_place = Some(10);

    match race_place {
        Some(1) => println!("You're number one!"),
        Some(2..=3) => println!("So close!"),
        Some(4..=9) => println!("{}th place, not bad...", race_place.unwrap()),
        Some(_) => println!("At least you finished the race!"),
        None => println!("DNF, you suck :P"),
    }
    
    //Matches are great for patterns because they MUST be exhaustive:
        //It's an error to have a match that doesn't account for all possible values of what's being matched
}

fn if_else_while_let() {
    //It's a shorthand for matches in exchange for being less safe
    let a = Some(vec![1, 2, 3]);

    if let Some(any) = a {
        let mut iter = any.iter();

        //let statements can also shadow stuff: we shadowed a
        while let Some(a) = iter.next() {
            if *a > 2 {
                println!("big number here!");
            } else if let 1 = a {
                println!("it's 1!");
            }
        }
    } else {
        println!("No vec");
    }
}

fn for_loops() {
    //For loops actually pattern match too
    println!("Cooking up devious iterator...");
    let vec = vec![1, 13, 156, 12, 19, 11];

    let a = vec.iter().enumerate()
        .map(|(i, val)| Some((i * val + 3, val.checked_div(i).or(Some(10)))))
        .zip((0..1000).rev());

    for (some, num) in a {
        println!("{some:?}, {num:?}");
    }
}

fn let_assignment() {
    //Yes, let assignments are patterns.
    let _x = 10;
    let (_a, _b, _c) = (1, 2, 3);

    //this will fail if you don't match correctly
    // let (_will, _fail): (&str, u8) = (true, 100, 1u8);

    //We'll learn how to let this pass in another section
}

fn fn_params(x: usize, y: &str, z: &Vec<String>) -> bool {
    if (y.len() > x) && (z.last().unwrap_or(&" ".to_string()).bytes().last().unwrap() > 10) {
        true
    } else {
        false
    }
}
