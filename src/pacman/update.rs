pub mod pacman_update{
    use std::{io::{stdin, stdout, Write}, process::Command};

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
            pacman(true);
        }else {
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();

            println!("Update Cancel \n");
        }
    }

    fn pacman(is_async: bool){
        if is_async {
            let mut child_process = Command::new("sudo")
            .args(&["pacman", "-Syu", "--noconfirm"])
            .spawn()
            .expect("Failed Update Pacman");

            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();

            child_process.wait().expect("Failed to wait for child process");
            println!("Pacman UPDATED\n");
        }else {
            let update = Command::new("sudo")
            .args(&["pacman", "-Syu", "--noconfirm"])
            .output()
            .expect("Error Update Pacman");

            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();

            println!("{}Pacman UPDATED\n", String::from_utf8_lossy(&update.stdout));
            println!("");
        }
    }
}