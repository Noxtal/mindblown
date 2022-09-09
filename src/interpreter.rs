use std::io::{self, Write};

use snailquote::unescape;

use crate::parser::Node;

pub struct Interpreter {
    tape: Vec<u8>,
    cursor: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            tape: vec![0; 30000],
            cursor: 0,
        }
    }

    fn shift(&mut self, offset: &isize) {
        self.cursor = ((30000 + self.cursor as isize + *offset) % 30000) as usize;
    }

    fn edit(&mut self, amount: &isize) {
        self.tape[self.cursor] = ((256 + self.tape[self.cursor] as isize + *amount) % 256) as u8;
    }

    pub fn interpret(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            match node {
                Node::Edit(amount) => {
                    self.edit(amount);
                }
                Node::Shift(amount) => {
                    self.shift(amount);
                }
                Node::Scan => {
                    print!("\n({})> ", self.cursor);
                    io::stdout().flush().unwrap();
                    let mut inp = String::new();
                    io::stdin().read_line(&mut inp).unwrap();
                    self.tape[self.cursor] = unescape(&inp).unwrap().chars().nth(0).unwrap() as u8;
                    println!("{}", self.tape[self.cursor]);
                }
                Node::Print => {
                    print!("{}", self.tape[self.cursor] as char);
                }
                Node::Loop(nodes) => {
                    while self.tape[self.cursor] != 0 {
                        self.interpret(&nodes);
                    }
                }
                Node::Clear => self.tape[self.cursor] = 0,
                Node::Add(offset) => {
                    let cur = self.tape[self.cursor];
                    self.shift(offset);
                    self.edit(&(cur as isize));
                    self.shift(&-offset);
                    self.tape[self.cursor] = 0;
                }
            }
        }
    }
}
