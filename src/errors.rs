use std::process::exit;
use colored::*;

fn err(etype: &str, msg: &str) {
    eprintln!("{} {} {}", "[mindblown]".magenta(), format!("{} ERROR", etype).red().bold(), msg);
    exit(1);
}

fn syntax_error(msg: &str) {
    err("SYNTAX", msg);
}

fn system_error(msg: &str) {
    err("SYSTEM", msg);
}

pub(crate) fn unbalanced_brackets() {
    syntax_error("Unbalanced square brackets! Make sure the count of '[' and ']' is the same.");
}

pub(crate) fn file_not_found(path: &str) {
    system_error(&format!("File \"{}\" was not found! Make sure this file exists.", path));
}

pub(crate) fn program_failure(pgms: &str) {
    system_error(&format!("Failed to run program(s) \"{}\". Make sure those are installed and comprised in your PATH.", pgms));
}