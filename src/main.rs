use std::io;
use std::process::exit;
use std::io::Write;
use std::env;
use std::path::Path;
use std::os::fd::AsFd;
use std::str;
use std::fs;
use std::collections::HashMap;
use rsh::lexer::Lexer;
use rsh::line::Line;
use rsh::bytecode::{Chunk, OpCode, ValueType};
use rsh::vm::VM;
use rsh::tokens::TokenType;

use strum::IntoEnumIterator;

fn main() {
 

    let mut vm = VM::new();
    let args: Vec<String> = env::args().collect();
//    println!("{:?}", args);
    if args.len() > 2 {
        println!("usage: rsh [*.yass]");
       // exit(1);
    }
        

    else if args.len() == 2 {

        let source_text = fs::read_to_string(&args[1]).expect("Failed to read file");


 //       VM::init();

        //let mut chunk_stack: Vec<Chunk> = Vec::new();
        //chunk_stack.push(OpCode::RETURN);
        

//        println!("{:?}", test_chunk);

        //chunk_stack.push(OpCode::CONSTANT);
        //test_chunk.disassemble_chunk();        
        
        vm.interpret(&source_text);

        //disassembleChunk(&chunk);
        //interpret(&chunk);

//        VM::free();
        exit(0);
    }


    
    match env::var("USER") {
        Ok(usrname) => {
                        println!("Welcome, \x1b[3m\x1b[38;5;141m{}\x1b[0m", usrname);
        }
        Err(_error) => {panic!("Failed to get user's name");}
    };


 
    //let mut line_stack = Vec::new();

    loop {
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

        //line_stack.push(line.clone());
        vm.interpret(&line);
        vm.free();


    //    let split_lines: Vec<String> = line.trim().split(';').map(|linestr: &str| linestr.to_string()).collect(); 
      //  for line in split_lines {
        //    let line_struct = Line::new(&line.clone());
//
   //         let stioh = io::stdin();
     //       let _infd = stioh.as_fd();
    //        println!("infd: {:?}", infd);
            
            
       //     line_struct.execute_line();
        //}

    }
}
