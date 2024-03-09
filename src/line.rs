use std::process;
use std::process::Stdio;
use std::process::Child;
use crate::command::Command as mycmd;

#[derive(Debug)]
pub struct Line {

   text: String,
   commands: Vec<mycmd>,

}


impl Line {
    pub fn new(raw_line: &str) -> Self {
        let trimmed = raw_line.trim();
        Self {

            text: trimmed.to_string(),
            commands: Self::generate_commands(trimmed),

        }
    }
    

    pub fn generate_commands(text: &str) -> Vec<mycmd> {
        let cv: Vec<mycmd> = text.split('|').map(|token| mycmd::new(token)).collect();

        return cv;
    }
    pub fn execute_line(mut self) {
        if self.commands.len() == 1 {
            self.commands[0].match_execute();
            return;
        }


        let mut previous_command = None;
        for index in 0..self.commands.len() {
            let mut com_obj = process::Command::new(&self.commands[index].cmd);
            com_obj.args(&self.commands[index].args);
            if index == 0 {
                match &self.commands[index].instream {
                    None => (),
                    Some(f) => {
                        com_obj.stdin(f.try_clone().unwrap());
                    }
                };

            }
            if index == self.commands.len()-1 {
                match &self.commands[index].outstream {
                    None => (),
                    Some(f) => {
                        com_obj.stdout(f.try_clone().unwrap());
                    }
                };
            }
            if index != self.commands.len()-1 {
                com_obj.stdout(Stdio::piped());
            }
            if index != 0 {
               // let mut previous_output = outputs[index-1].stdout;
                com_obj.stdin(previous_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()))); 
                //match &outputs[index-1].stdout {
                  //  None => (),
                    //Some(output) => {com_obj.stdin(*output);},
                //}
            }
            let cochild = com_obj.spawn();
            if index == self.commands.len()-1 {
                match cochild {
                    Ok(mut childp) => {
                        previous_command = None;
                        childp.wait().unwrap();
                    },
                    Err(e) => {previous_command = None;

                        eprintln!("{}", e);},
                };
            }
            else {
                match cochild {
                    Ok(childp) => {
                        previous_command = Some(childp);
                    },

                    Err(e) => {previous_command = None;
                        eprintln!("{}", e);
                    },


                };
            }
        }
    }


    pub fn get_commands(self) -> Vec<mycmd> {
        return self.commands;        
    }
}
