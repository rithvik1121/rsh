use std::io;
//use std::process::Command;
use std::io::Write;
use std::env;
use std::os::fd::AsFd;
mod command;
mod line;

fn main() {
 //   println!("Hello, world!");
 
    let mut line_stack = Vec::new();

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
        let line_struct = line::Line::new(&line.clone());
   //     println!("{:?}", line_struct);

        let stioh = io::stdin();
        let infd = stioh.as_fd();
//        println!("infd: {:?}", infd);
        
  //      println!("commmand stak: {:?}", line_stack);
        
        line_struct.execute_line();

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
