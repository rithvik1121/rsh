use std::fs::File;
use std::path::PathBuf;
use std::process;
use std::env;

pub struct Command{
    cmd: String, 
    args: Vec<String>,
    instream: Option<File>,
    outstream: Option<File>,
}

impl Command{
    pub fn new(raw: &str) -> Self {
        let mut argv: Vec<&str> = raw.trim().split(' ').collect();
        //println!("{:?}", argv);
        Self {
            cmd: argv[0].to_string(),
            args: argv[1..].iter().map(|arg| arg.to_string()).collect(),
            instream: None,
            outstream: None,
        }
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
            _ => {
                //println!("{}", self.cmd);
                //println!("{:?}", self.args);

                let mut cmd = process::Command::new("cmd").arg("/c").arg(self.cmd).args(self.args).spawn().unwrap();
                cmd.wait().expect("Failed to execute command");
            }
        }
    }
}


