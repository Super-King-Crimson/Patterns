pub mod match_guards;
pub mod multiple_patterns;
pub mod destructures;
pub mod unused;
pub mod at_bindings;

pub fn explain() {
    println!("Welcome to the pattern syntax dump"); 

    println!("On second thought, why should I write all of this?");
    println!("GET OUTTA HERE! GO READ CHAPTER 18.3!");
    println!("https://rust-book.cs.brown.edu/ch18-03-pattern-syntax.html");

    println!("...I will write some notable cases down though.");

    multiple_patterns::explain();
    destructures::explain();
    unused::explain();
    match_guards::explain();
    at_bindings::explain();
}