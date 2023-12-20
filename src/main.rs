use std::io;
//use std::process::Command;
use std::io::Write;
use std::env;
use std::os::fd::AsFd;
mod command;

fn main() {
 //   println!("Hello, world!");
 
    let mut command_stack = Vec::new();

    loop {
        let mut line = String::new();
        //let mut args;
        let path = match env::current_dir(){
            Ok(p) => p,
            Err(_error) => panic!("Could not get current directory"),
        };
        print!("\x1b[4m\x1b[38;5;45mrsh\x1b[0m {}> ", path.display());
        io::stdout().flush().expect("Failed to flush stdio");
        //event loop?
        //
        //
        io::stdin().read_line(&mut line).expect("Failed to read line");
        line_stack.push(line.clone());
        let com = command::Command::new(&mut line);
        let stioh = io::stdin();
        let infd = stioh.as_fd();
        println!("{:?}", infd);
        
        println!("commmand stak: {:?}", line_stack);
        
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
