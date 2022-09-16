use rand::{distributions::Alphanumeric, Rng};
use std::{
    env,
    fs::{self, remove_file, File},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};
use subprocess::{Popen, PopenConfig};

use clap::{arg, command, Command as ClapCommand};

mod parser;
use crate::parser::*;

mod interpreter;

mod codegen;

fn main() {
    let matches = command!()
        .subcommand(
            ClapCommand::new("compile")
                .about("Compiles a Brainfuck file.")
                .arg(
                    arg!(<FILE>)
                        .help("The Brainfuck file to compile.")
                        .required(true),
                )
                .arg(
                    arg!([OUTPUT])
                        .short('o')
                        .long("output")
                        .help("The output file.")
                        .takes_value(true),
                )
                .arg(
                    arg!([RUN])
                        .short('r')
                        .long("run")
                        .help("Run the compiled executable after compiling.")
                        .takes_value(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("compile", sub_matches)) => {
            if let Some(path) = sub_matches.get_one::<String>("FILE") {
                let outfile = compile(&path, sub_matches.get_one("OUTPUT"));
                if sub_matches.is_present("RUN") {
                    run(outfile);
                }
            }
        }
        _ => repl(),
    }

    if cfg!(windows) {
        Command::new("wsl")
            .arg("--shutdown")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
    }
}

fn windir_to_wsl(windir: &str) -> String {
    let mut wsl = String::from("/mnt/");
    wsl.push_str(&windir[0..1].to_lowercase());
    wsl.push_str(&windir[2..]);
    wsl.replace("\\", "/")
}

fn compile(path: &str, output: Option<&String>) -> String {
    use crate::codegen::*;

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

    let outpath = match output {
        Some(o) => Path::new(o).to_path_buf(),
        None => {
            let mut o = Path::new(&path).to_path_buf();
            o.set_extension("out");
            o
        }
    }
    .to_str()
    .unwrap()
    .to_string();

    let out = if cfg!(target_os = "windows") {
        Command::new("bash")
            .arg("-c")
            .arg(format!(
                "gcc -nostdlib -Wa,-msyntax=intel,-mnaked-reg {} -static -o \"{}\"",
                windir_to_wsl(tempdir.to_str().unwrap()),
                &outpath
            ))
            .output()
            .expect(
                "Failed to execute WSL bash/GCC: Make sure both are installed and in your PATH!",
            )
    } else {
        Command::new("gcc")
            .arg("-nostdlib")
            .arg("-Wa,-msyntax=intel,-mnaked-reg")
            .arg(tempdir.to_str().unwrap())
            .arg("-static")
            .arg("-o")
            .arg(&outpath)
            .output()
            .expect("Failed to execute GCC: Make sure it is installed and in your PATH!")
    };

    remove_file(tempdir).unwrap();

    if out.stderr.len() == 0 {
        println!("[mindblown] Compiled successfully!");
        println!("{}", String::from_utf8_lossy(&out.stdout));
    } else {
        println!("[mindblown] Compilation failed!");
        println!("{}", String::from_utf8_lossy(&out.stderr));
    }

    outpath
}

fn run(path: String) {
    let windows = cfg!(target_os = "windows");

    let path = dunce::canonicalize(path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if windows {
        Popen::create(
            &["bash", "-c", &windir_to_wsl(&path)],
            PopenConfig {
                ..Default::default()
            },
        )
    } else {
        Popen::create(
            &[path],
            PopenConfig {
                ..Default::default()
            },
        )
    }
    .unwrap();
}

fn repl() {
    use crate::interpreter::*;

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
