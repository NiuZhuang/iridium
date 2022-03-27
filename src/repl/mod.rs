use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

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
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing");
                }
                _ => {
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte);
                            }
                        }
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.");
                        }
                    }
                }
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    ///
    /// Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
