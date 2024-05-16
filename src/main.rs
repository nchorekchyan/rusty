//use std::io;
use dialoguer::Select;

use rusty::dummy_window;

fn main() {

    //build the basic window
    dummy_window();

    //game that needs to be moved into window somehow
    println!("Welcome to Mordavia");

    let choice = vec!["Val","Ezekeal","Chowabunga","Serenety"];

    let select = Select::new()
        .with_prompt("Choose your fate")
        .items(&choice)
        .interact()
        .unwrap();

    println!("I see you choose {}, Pathetic", choice[select]);
    
}

