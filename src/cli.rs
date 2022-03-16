extern crate colour;
use colour::*;

use std::env;
// use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // let filename: &String = &args[1];
    
    // let filename: &String = &args[2];
    let option: &String = &args[1];
    if option == "yes" {
        println!("hi");
    }
    // let content = fs::read_to_string(filename).expect("Unable to read file");
    // println!("{}", content);
}

// pub fn foo() {
//     let err: Result<(), u8> = Err(1);
//     prnt_ln!("Failed on {}", 9);
//     yellow!("Error details: ");
//     red_ln!("{:?}", err);
// }

pub fn all_colours() {
    black!("black ");
    red!("red ");
    green!("green ");
    yellow!("yellow ");
    blue!("blue ");
    magenta!("magenta ");
    cyan!("cyan ");
    white!("white ");
    // dark_black!("dark_black ");
    dark_red!("dark_red ");
    dark_green!("dark_green ");
    dark_yellow!("dark_yellow ");
    dark_blue!("dark_blue ");
    dark_magenta!("dark_magenta ");
    dark_cyan!("dark_cyan ");
    // dark_white!("dark_white ");
    prnt!("default colour\n\n");
}