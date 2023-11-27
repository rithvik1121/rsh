use std::process;
use std::process::Command;
use std::env;
//use std::path::Path;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs::File;
//pub mod cmstruct;

pub fn split_pipes(input: &mut str) {
    
}
pub fn tokenize_command(input: &mut str) -> Vec<&str> {
    //println!("{:?}", handle_redirects(input));    
//    println!("{:?}", parse_for_redirect(input));
    let mut refiles = [None, None];
    let redirloc = parse_for_redirect(input);
    let &mut combloc;
    let mut argv: Vec<&str>;

    if redirloc > 0 {
        refiles = handle_redirects(input);
        combloc = &input[0..redirloc];
    }
    else {
        combloc = input;
    } 
 //   println!("{:?}", combloc);
    argv = combloc.split(' ').collect(); 
 //   println!("{:?}", argv);
    return argv;
}

pub fn parse_for_redirect(input: &mut str) -> usize {
        match input.find('<') {
            None => match input.find('>') {
                        None => 0,
                        Some(i) => i,
                    },
            Some(l) => l,
        }
}

pub fn handle_redirects(input: &mut str) -> [Option<File>; 2]{
   let mut inredir: Vec<&str> = input.split('<').collect();
   let outredir: Vec<&str> = input.split('>').collect();
   let mut infile = None;
   let mut outfile = None;

   if inredir.len() > 1 {
        //println!("{}", inredir[1]);
        let chainoutput: Vec<&str> = inredir[1].split('>').collect();
        if chainoutput.len() > 1 {
//            println!("chained output: {:?}", chainoutput);
            inredir[1] = chainoutput[0];
        } 
        let inop = OpenOptions::new().read(true).open(inredir[1].trim());
        match inop {
            Err(_e) => println!("Failed to open file {} for reading", inredir[1].trim()),
            Ok(f) => infile = Some(f),
        }; 
    }

   if outredir.len() > 1 {
        //println!("{}", outredir[1]);
        let outop = OpenOptions::new().read(true).write(true).truncate(true).create(true).open(outredir[1].trim());
        match outop {
            Err(_e) => println!("Failed to open file {} for writing", outredir[1].trim()),
            Ok(f) => outfile = Some(f),
        };
    }
   [infile, outfile]
       
}

// pub fn match_execute(argv: &Vec<&str>) {
//     //println!("{:?}", argv);
//     match argv[0] {
//         "cd" => {
//             let mut new_dir = env::current_dir().unwrap();
//             if (argv.len() > 2) {
//                new_dir = PathBuf::from(argv[1]);
//             }
//             if let Err(e) = env::set_current_dir(&new_dir) {
//                 eprintln!("{}", e);
//             }
//         }
//         "exit" => process::exit(0),
//         _ => {
//             let mut cmd = Command::new("cmd").arg("/c").args(argv).spawn().unwrap();
//             cmd.wait().expect("Failed to execute command");
//         }
//     };
// }
