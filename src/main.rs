use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::env;

pub struct Bf {
    pc: usize,

    tape: [u8; 30000],
    ptr: usize,

    stack: Vec<usize>,
    braces: [usize; 30000],

    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl Bf {
    pub fn new() -> Self {
        Bf {
            pc: 0,
            tape: [0; 30000],
            ptr: 0,
            stack: vec![],
            braces: [0; 30000],
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    pub fn run(&mut self, instructions: &str) -> Result<(), String> {
        let chars = instructions.as_bytes();
        
        // match the braces first
        self.pc = 0;
        loop {
            let instruction = chars[self.pc];
            if instruction == '[' as u8 {
                self.stack.push(self.pc);
            }
            else if instruction == ']' as u8 {
                if let Some(open) = self.stack.pop() {
                    self.braces[self.pc] = open;
                    self.braces[open] = self.pc;
                }
                else {
                    return Err(format!("unmatched ']' at byte {}", self.pc));
                }
            }

            self.pc += 1;
            if self.pc == chars.len() {
                if let Some(open) = self.stack.pop() {
                    return Err(format!("unmatched '[' at byte {}", open));
                }
                break;
            }
        }

        // now execute
        self.pc = 0;
        loop {
            let instruction = chars[self.pc];
            self.execute(instruction);
             
            self.pc += 1;
            if self.pc == chars.len() {
                break;
            }
        }
        Ok(())
    }

    fn execute(&mut self, instruction: u8) {
        match instruction as char {
            '>' => { 
                self.ptr = self.ptr.wrapping_add(1);
            },
            '<' => {
                self.ptr = self.ptr.wrapping_sub(1);
            },
            '+' => {
                self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1);
            }
            '-' => {
                self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1);
            },
            '.' => {
                let ch = {
                    if self.tape[self.ptr] == 10 {
                        '\n' as u8
                    }
                    else {
                        self.tape[self.ptr]
                    }
                };
                self.stdout.write(&vec![ch]).expect("stdout write failed");
            },
            ',' => {
                let mut buf: [u8; 1] = [0];
                self.stdin.read(&mut buf).expect("stdin read failed");
                
                self.tape[self.ptr] = {
                    if buf[0] as char == '\n' {
                        10
                    }
                    else {
                        buf[0]
                    }
                }
            },
            '[' => {
                if self.tape[self.ptr] == 0 {
                    self.pc = self.braces[self.pc];
                }
            },
            ']' => {
                if self.tape[self.ptr] != 0 {
                    self.pc = self.braces[self.pc];
                }
            }
            _ => { /* it's a comment */ }
        }
    }
}

fn main() {
    let mut bf = Bf::new();
    let mut buf = String::new();
    let file = env::args().nth(1).expect("no file supplied");
    File::open(file)
        .expect("file open failed")
        .read_to_string(&mut buf)
        .expect("file read failed");

    if let Err(e) = bf.run(&buf) {
        println!("run failed: {}", e);
    }
}