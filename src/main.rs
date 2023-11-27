use std::io;
//use std::process::Command;
use std::io::Write;
use std::env;
mod auxiliary;
mod command;
mod line;

fn main() {
 //   println!("Hello, world!");

    loop {
        let mut line = String::new();
        //let mut args;
        let path = match env::current_dir(){
            Ok(p) => p,
            Err(_error) => panic!("Could not get current directory"),
        };
        print!("\x1b[4m\x1b[38;5;45mrsh\x1b[0m {}> ", path.display());
        io::stdout().flush().expect("Failed to flush stdio");
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let com = command::Command::new(&mut line);
        
        //args = auxiliary::tokenize_command(&mut line);
        com.match_execute(); 
        //pipes
        //
        //
        //input/output redirection
        //
        //
        //execute

//        let argv: Vec<&str> = args.split_whitespace().collect();
  //      auxiliary::match_execute(&argv); 

    }
}
