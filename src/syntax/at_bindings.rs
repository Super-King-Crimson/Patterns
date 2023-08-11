pub fn explain() {
    //Allows for creation of a variable that holds a value that's also being pattern matched
    let hi = Hello { id: 10 };

    match hi {
        Hello { id: id_var @ 3..=7 } => println!("id in range 3 - =7: {id_var}"),
        Hello { id: 1000..=10000 } => println!("id is in range 1000 - =10000"),
        _ => (),
    }
}

use Message::*;
enum Message {
    Hello { id: i32 },
}