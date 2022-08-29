use std::{
    io::{self, Write},
    iter::Peekable,
    str::Chars,
};

fn main() {
    println!("MINDBLOWN {} - BRAINF**K REPL", env!("CARGO_PKG_VERSION"));

    // Read-Eval-Print Loop
    let mut interpreter = Interpreter::new();
    loop {
        print!("mindblown> ");
        io::stdout().flush().unwrap();
        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();
        if code.contains("exit") || code.contains("quit") {
            println!("Goodbye!");
            break;
        }
        let nodes = Parser::parse(&code);
        println!("{:?}", nodes);
        interpreter.interpret(&nodes)
    }
}

#[derive(Debug)]

enum Node {
    Edit(isize),
    Shift(isize),
    Scan,
    Print,
    Loop(Vec<Node>),
    Clear,
    Move(isize),
}

struct Parser<'a> {
    iterator: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn parse(input: &'a String) -> Vec<Node> {
        let mut parser = Parser {
            iterator: input.chars().peekable(),
        };
        parser._parse()
    }

    fn _parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while let Some(c) = self.iterator.next() {
            match c {
                '+' | '-' => {
                    let mut acc = if c == '+' { 1 } else { -1 };
                    while let Some(node) = self.iterator.peek() {
                        match node {
                            '+' => acc += 1,
                            '-' => acc -= 1,
                            _ => break,
                        }

                        self.iterator.next();
                    }
                    if acc != 0 {
                        nodes.push(Node::Edit(acc));
                    }
                }
                '<' | '>' => {
                    let mut acc = if c == '>' { 1 } else { -1 };
                    while let Some(node) = self.iterator.peek() {
                        match node {
                            '>' => acc += 1,
                            '<' => acc -= 1,
                            _ => break,
                        }

                        self.iterator.next();
                    }
                    if acc != 0 {
                        nodes.push(Node::Shift(acc));
                    }
                }
                '.' => nodes.push(Node::Print),
                ',' => nodes.push(Node::Scan),
                '[' => {
                    let contents = self._parse();
                    if !contents.is_empty() {
                        if contents.len() == 1 {
                            if let Node::Edit(amount) = contents[0] {
                                if amount.abs() == 1 {
                                    nodes.push(Node::Clear);
                                    continue;
                                }
                            }
                        }

                        if contents.len() == 4 {
                            if let (
                                Node::Edit(e1),
                                Node::Shift(s1),
                                Node::Edit(e2),
                                Node::Shift(s2),
                            ) = (&contents[0], &contents[1], &contents[2], &contents[3])
                            {
                                if e1.abs() == 1 && e1 == &-e2 && s1 == &-s2 {
                                    nodes.push(Node::Move(*s1));
                                    continue;
                                }
                            }
                        }

                        nodes.push(Node::Loop(contents));
                    }
                }
                ']' => break,
                _ => (),
            }
        }
        nodes
    }
}

struct Interpreter {
    tape: Vec<u8>,
    cursor: usize,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            tape: vec![0; 30000],
            cursor: 0,
        }
    }

    fn shift(&mut self, offset: &isize) {
        self.cursor = ((30001 + self.cursor as isize + *offset) % 30001) as usize;
    }

    fn interpret(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            match node {
                Node::Edit(amount) => {
                    self.tape[self.cursor] =
                        ((256 + self.tape[self.cursor] as isize + *amount) % 256) as u8;
                }
                Node::Shift(amount) => {
                    self.shift(amount);
                }
                Node::Scan => {
                    print!("({})> ", self.cursor);
                    io::stdout().flush().unwrap();
                    let mut inp = String::new();
                    io::stdin().read_line(&mut inp).unwrap();
                    self.tape[self.cursor] = inp.chars().nth(0).unwrap() as u8;
                    println!();
                }
                Node::Print => {
                    print!("{}\n", self.tape[self.cursor] as char);
                }
                Node::Loop(nodes) => {
                    while self.tape[self.cursor] != 0 {
                        self.interpret(&nodes);
                    }
                }
                Node::Clear => self.tape[self.cursor] = 0,
                Node::Move(offset) => {
                    let cur = self.tape[self.cursor];
                    self.tape[self.cursor] = 0;
                    self.shift(offset);
                    self.tape[self.cursor] = cur;
                }
            }
        }
    }
}
