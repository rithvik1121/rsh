use std::io;
use std::process::exit;
use std::io::Write;
use std::env;
//use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::os::fd::AsFd;
use std::str;
use std::fs;
use rsh::line::Line;
use rsh::vm::VM;
//use rsh::environment::Env;
use rsh::values::ValueType;

fn main() {
 

    let mut vm = VM::new();
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if args.len() > 3 {
        println!("usage: rsh [*.yass] [--debug]");
        exit(1);
    }
        

    if args.len() == 2 {

        let source_text = fs::read_to_string(&args[1]).expect("Failed to read file");


        vm.interpret(&source_text);
        vm.free();

        exit(0);
    }

    if args.len() == 3 {

        if &args[2]=="--debug" {
            vm.env.debug=true;
        }
        else {
            println!("Invalid flag");
            exit(2);
        }

        let source_text = fs::read_to_string(&args[1]).expect("Failed to read file");


        vm.interpret(&source_text);
        vm.free();

        exit(0);
    }
    
    match env::var("USER") {
        Ok(usrname) => {
                        println!("Welcome, \x1b[3m\x1b[38;5;141m{}\x1b[0m", usrname);
        }
        Err(_error) => {panic!("Failed to get user's name");}
    };


    let rcpath = env::var("HOME").unwrap() + "/.rshrc";
    let rshrc = fs::read_to_string(rcpath);

    match rshrc {
        Ok(src) => vm.interpret(&src),
        Err(_e) => None,
    };
    //vm.interpret( 
 
    let mut line_stack = Vec::new();
    loop {
        vm.command_mode = true;
        let mut line = String::new();
        //let mut args;
        let mut path = match env::current_dir(){
            Ok(p) => p,
            Err(_error) => panic!("Could not get current directory"),
        };
        match path.strip_prefix(env::var("HOME").unwrap()) {
            Ok(p) => {path = Path::new("~").to_path_buf().join(p.to_path_buf());},
            Err(_e) => (),

        };
        print!("\x1b[4m\x1b[38;5;45mrsh\x1b[0m {} -> ", path.display());
        io::stdout().flush().expect("Failed to flush stdio");
        io::stdin().read_line(&mut line).expect("Failed to read line");

        
        for key in vm.env.vars.keys() {
            line = line.replace(key, &ValueType::extract_str(vm.env.vars[key].value.clone()).unwrap());
        }

        line_stack.push(line.clone());



        let split_lines: Vec<String> = line.trim().split(';').map(|linestr: &str| linestr.to_string()).collect(); 
        for line in split_lines {
            let mut line_struct = Line::new(&line.clone());

            let stioh = io::stdin();
            let _infd = stioh.as_fd();
            //println!("infd: {:?}", infd);
            
            
            line_struct.execute_line();
        }
    
    }

}
