mod util;
mod pacman;

use std::io::{stdout, Write};

use util::input::input_util::input_user;
use pacman::install::pacman_install::install_package;
use pacman::update::pacman_update::update_pacman;

fn main(){
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
    loop {
        let menu = "WELCOME TO PACMAN!!! \n\
        1. Install Package \n\
        2. Update Pacman \n\
        X. Exit";
        println!("{menu}");

        print!("Pilih menu: ");
        let input = input_user();

        if input == "1" {
            print!("\x1B[2J\x1B[1;1H");
            // stdout().flush().unwrap();

            install_package();
        }else if input == "2" {
            print!("\x1B[2J\x1B[1;1H");
            // stdout().flush().unwrap();

            update_pacman();
        }else {
            if input.to_lowercase() == "x" {
                break;
            }
            print!("\x1B[2J\x1B[1;1H");
            // stdout().flush().unwrap();
            println!("Input yang bener bangsat\n");
        }
    }
    println!("Exit dari PACMAN!!!");
}