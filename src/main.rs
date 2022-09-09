use rand::{distributions::Alphanumeric, Rng};
use std::{
    env,
    fs::{self, remove_file, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

mod parser;
use crate::parser::*;

mod interpreter;
use crate::interpreter::*;

mod codegen;
use crate::codegen::*;

fn main() {
    println!(
        "MINDBLOWN {} - BRAINF**K INTERPRETER/COMPILER",
        env!("CARGO_PKG_VERSION")
    );

    match env::args().nth(1) {
        Some(path) => {
            compile(&path);
        }
        None => {
            repl();
        }
    }
}

fn compile(path: &str) {
    let contents = fs::read_to_string(&path).expect("Could not read specified file/path!");
    let nodes = Parser::parse(&contents);
    let asm = Codegen::generate(&nodes);

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

fn repl() {
    let mut interpreter = Interpreter::new();
    loop {
        print!("mindblown> ");
        let mut code = String::new();

        loop {
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut code).unwrap();
            code = code.trim().to_string();

            if code.matches('[').count() == code.matches(']').count() {
                break;
            }

            print!("> ");
        }

        if code.contains("exit") || code.contains("quit") {
            println!("Goodbye!");
            break;
        }
        let nodes = Parser::parse(&code);
        interpreter.interpret(&nodes)
    }
}