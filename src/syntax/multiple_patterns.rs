pub fn explain() {
    let x = Some(10);

    match x {
        Some(1) | Some(2) | Some(3) => println!("Hey, top 3!"),
        _ => println!("L"),
    }
}