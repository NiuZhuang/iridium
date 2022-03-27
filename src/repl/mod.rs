use crate::vm::VM;
use std;
use std::io;
use std::io::Write;

/// Core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    /// The VM the REPL will use to execute code
    vm: VM,
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");

        loop {
            // TODO: 在循环之外创建 buffer，每个迭代都能重复使用
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!(">>> ");

            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    print!("Farewell! Have a good day!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    println!("Invalid input")
                }
            }
        }
    }
}
