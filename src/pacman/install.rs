pub mod pacman_install{
    use std::{io::{stdin, stdout, Write}, process::Command};

    pub fn install_package(){
        println!("WELCOME TO INSTALL PACKAGE");
        print!("Ketik package yang ingin anda install (x untuk cancel): ");
        stdout().flush().unwrap();

        let mut package = String::new();
        stdin().read_line(&mut package).unwrap();
        let package_input: Result<String, _> = package.trim().parse();

        if package_input.unwrap().to_lowercase() == "x"{
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();

            println!("Install Cancel \n");
        }else {
            pacman(true, package)
        }
    }

    fn pacman(is_async: bool, package: String){
        if is_async{
            let mut install = Command::new("sudo")
            .args(&["pacman", "-S", &package.trim(), "--noconfirm"])
            .spawn()
            .unwrap();

            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();

            install.wait().expect("Installation Failed");
            println!("");
        }else {
            let install = Command::new("sudo")
            .args(&["pacman", "-S", &package.trim(), "--noconfirm"])
            .output()
            .unwrap();

            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();

            if install.status.success(){
                let output_ls = String::from_utf8_lossy(&install.stdout);
                println!("{}", output_ls);
            }else {
                let output_ls = String::from_utf8_lossy(&install.stderr);
                println!("{}", output_ls);
            }
        }
    }
}