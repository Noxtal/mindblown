use rand::{distributions::Alphanumeric, Rng};
use std::{
    env,
    fs::{self, remove_file, File},
    io::{self, Write},
    iter::Peekable,
    path::Path,
    process::Command,
    str::Chars,
};

use snailquote::unescape;

fn main() {
    println!(
        "MINDBLOWN {} - BRAINF**K INTERPRETER/COMPILER",
        env!("CARGO_PKG_VERSION")
    );

    match env::args().nth(1) {
        Some(path) => {
            let contents = fs::read_to_string(&path).expect("Could not read specified file/path!");
            let nodes = Parser::parse(&contents);
            let asm = Compiler::assemble(&nodes);

            let mut tempdir = std::env::temp_dir();
            let mut filename: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
            filename += ".s";
            tempdir.push(filename);

            File::create(&tempdir)
                .unwrap()
                .write_all(asm.as_bytes())
                .unwrap();

            let out = if cfg!(target_os = "windows") {
                let mut wsltempdir = tempdir.to_str().unwrap().replace("\\", "/").replace(":", "");
                let mut v: Vec<char> = wsltempdir.chars().collect();
                v[0] = v[0].to_lowercase().nth(0).unwrap();
                wsltempdir = "/mnt/".to_string() + &v.into_iter().collect::<String>();
                
                Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "gcc -nostdlib -Wa,-msyntax=intel,-mnaked-reg {} -static -o \"{}\"",
                    wsltempdir,
                    Path::new(&path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".b", "")
                ))
                .output()
                .expect("Failed to execute WSL bash/GCC: Make sure both are installed and in your PATH!")
            } else {
                Command::new("gcc")
                        .arg("-nostdlib")
                        .arg("-Wa,-msyntax=intel,-mnaked-reg")
                        .arg(tempdir.to_str().unwrap())
                        .arg("-static")
                        .arg("-o")
                        .arg(
                            Path::new(&path)
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .replace(".b", ""),
                        )
                        .output()
                        .expect("Failed to execute GCC: Make sure it is installed and in your PATH!")
            };

            remove_file(tempdir).unwrap();

            if out.stderr.len() == 0 {
                println!("Compiled successfully!");
                println!("{}", String::from_utf8_lossy(&out.stdout));
            } else {
                println!("Compilation failed!");
                println!("{}", String::from_utf8_lossy(&out.stderr));
            }
        }
        None => {
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
                interpreter.interpret(&nodes)
            }
        }
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
    Add(isize),
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
                                Node::Shift(s1),
                                Node::Edit(e1),
                                Node::Shift(s2),
                                Node::Edit(e2),
                            ) = (&contents[0], &contents[1], &contents[2], &contents[3])
                            {
                                if e1.abs() == 1 && *e1 == -e2 && *s1 == -s2 {
                                    nodes.push(Node::Add(*s1));
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
        self.cursor = ((30000 + self.cursor as isize + *offset) % 30000) as usize;
    }

    fn edit(&mut self, amount: &isize) {
        self.tape[self.cursor] = ((256 + self.tape[self.cursor] as isize + *amount) % 256) as u8;
    }

    fn interpret(&mut self, nodes: &Vec<Node>) {
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

struct Compiler {
    loop_id: usize,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler { loop_id: 0 }
    }

    fn assemble(nodes: &Vec<Node>) -> String {
        let mut assembler = Compiler::new();
        format!(
            "# GNU Assembler, Intel syntax, x86_64 Linux
    
.data

.equ SYS_EXIT, 60
.equ SUCCESS, 0

.equ SYS_WRITE, 1
.equ STDOUT, 1

.equ SYS_READ, 0
.equ STDIN, 0 

.bss

.lcomm ARRAY, 30000

.text

.global _start

_start:
    mov r12, offset ARRAY
{}
    mov rax, SYS_EXIT
    mov rdi, SUCCESS
    syscall
",
            assembler._assemble(nodes)
        )
    }

    fn _assemble(&mut self, nodes: &Vec<Node>) -> String {
        let mut asm = String::new();
        for node in nodes {
            match node {
                Node::Edit(amount) => {
                    asm += &format!(
                        "
    {} [r12], {}",
                        if *amount > 0 { "addb" } else { "subb" },
                        amount.abs()
                    )
                }
                Node::Shift(amount) => {
                    asm += &format!(
                        "
    {} r12, {}",
                        if *amount > 0 { "add" } else { "sub" },
                        amount.abs()
                    )
                }
                Node::Scan => {
                    asm += "
    mov rax, SYS_READ
    mov rdi, STDIN
    mov rsi, r12
    mov rdx, 1
    syscall"
                }
                Node::Print => {
                    asm += "
    mov rax, SYS_WRITE
    mov rdi, STDOUT
    mov rsi, r12
    mov rdx, 1
    syscall"
                }
                Node::Loop(nodes) => {
                    self.loop_id += 1;
                    let id = self.loop_id;

                    asm += &format!(
                        "
    cmpb [r12], 0
    je LOOP_END_{}
    LOOP_START_{}:",
                        id, id
                    );

                    asm += &self._assemble(&nodes);
                    asm += &format!(
                        "
    cmpb [r12], 0
    jne LOOP_START_{}
    LOOP_END_{}:",
                        id, id
                    );
                }
                Node::Clear => {
                    asm += "
    movb [r12], 0"
                }
                Node::Add(offset) => {
                    asm += &format!(
                        "
    movb al, [r12]
    addb [r12 {}], al
    movb [r12], 0",
                        format!("{} {}", if *offset > 0 { "+" } else { "-" }, offset.abs())
                    )
                }
            };
        }
        asm
    }
}
