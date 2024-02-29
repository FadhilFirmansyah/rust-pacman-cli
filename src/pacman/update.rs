pub mod pacman_update{
    use std::io::{stdin, stdout, Write};

    pub fn update_pacman(){
        println!("WELCOME TO UPDATE PACMAN");
        print!("Apakah anda ingin update? (y/n): ");
        stdout().flush().unwrap();
    
        let mut input_update = String::new();
        stdin().read_line(&mut input_update).unwrap();
    
        let input: Result<char, _> = input_update.trim().parse();
        match input {
            Ok(_) => print!(""),
            Err(_) => None.unwrap()
        }
    
        if input.unwrap() == 'y' {
            //Do Something
        }else {
            println!("Cancel \n");
        }
    }
}