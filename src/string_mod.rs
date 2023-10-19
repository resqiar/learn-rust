pub fn main() {
    let mut greeting = String::from("Hai Dunia!");
    let another_greeting = " Apa kabar?!";

    greeting.push_str(another_greeting);
    println!("{greeting}");

    let x = String::from("A");
    let y = String::from("B");
    let z = x + &y;
    println!("{z}");

    let x = String::from("tic");
    let y = String::from("tac");
    let z = String::from("toe");
    let game = format!("{x}-{y}-{z}");
    println!("{game}");

    let x = String::from("книга"); // book

    // print as bytes
    for b in x.bytes() {
        println!("{b}");
    }

    // print as char
    for c in x.chars() {
        println!("{c}");
    }
}
