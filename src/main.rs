use std::io;


fn main() {

    println!("Welcome to Zaleria");

    println!("Choose your fate");


    let mut choice:String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("FAILURE");

    println!("Ahh, I see that you chose {choice}");

}
