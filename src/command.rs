use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process;
use std::env;

#[derive(Debug)]
pub struct Command{
    cmd: String, 
    args: Vec<String>,
    instream: Option<File>,
    outstream: Option<File>,
}

impl Command{
    pub fn new(raw: &str) -> Self {
        let outvec: Vec<&str> = raw.trim().split('>').collect();
        let invec: Vec<&str> = outvec[0].split('<').collect();
        let argv: Vec<&str> = invec[0].trim().split(' ').collect();
        
        let mut infile: Option<File> = None;
        let mut outfile: Option<File> = None;

        if outvec.len() > 1 {
            let outop = OpenOptions::new().read(true).write(true).truncate(true).create(true).open(outvec[1].trim());
            match outop {
                Err(_e) => println!("Failed to open file {} for writing", outvec[1].trim()),
                Ok(f) => outfile = Some(f),
            };
        }

        if invec.len() > 1 {
            let inop = OpenOptions::new().read(true).open(invec[1].trim());
            match inop {
                Err(_e) => println!("Failed to open file {} for reading", invec[1].trim()),
                Ok(f) => infile = Some(f),
            };
        }


//        println!("outvec: {:?}", outvec);
//        println!("invec: {:?}", invec);
//        println!("{:?}", argv);
//        println!("outfile: {:?}", outfile);
//        println!("infile: {:?}", infile);
        let this = Self {
            cmd: argv[0].to_string(),
            args: argv[1..].iter().map(|arg| arg.to_string()).collect(),
            instream: infile,
            outstream: outfile,
        };
        this
    }
    

    pub fn match_execute(self) {
        match self.cmd.as_str() {
            "cd" => {
                let mut new_dir = env::current_dir().unwrap();
                //println!("{:?}", self.args.len());
                if(self.args.len() > 0) {
                    new_dir = PathBuf::from(&self.args[0]);
                }
                if let Err(e) = env::set_current_dir(&new_dir) {
                    eprintln!("{}", e);
                }
            }
            "exit" => process::exit(0),
            "history" => auxiliary::history(),
            _ => {
                //println!("{}", self.cmd);
                //println!("{:?}", self.args);

                let mut cmd = process::Command::new(self.cmd);
                cmd.args(self.args);
                match self.instream {
                    None => (),
                    Some(f)  => {
                                 //println!("instream: {:?}", self.instream.unwrap());
                                 cmd.stdin(f);
                    },
                }
                match self.outstream {
                    None => (),
                    Some(f)  => {
                                 //println!("outstream: {:?}", self.outstream.unwrap());
                                 cmd.stdout(f);
                    },
                }
                let child = cmd.spawn();
                match child {
                    Ok(mut child) => {child.wait().unwrap();},
                    Err(e) => {eprintln!("{}", e);},
                };
            }
        }
    }
}


