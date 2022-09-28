use std::{iter::Peekable, str::Chars};

use crate::errors;

#[derive(Debug)]

pub enum Node {
    Edit(isize),
    Shift(isize),
    Scan,
    Print,
    Loop(Vec<Node>),
    Clear,
    Add(isize),
}

pub struct Parser<'a> {
    iterator: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn parse(input: &'a String) -> Vec<Node> {
        if input.matches('[').count() != input.matches(']').count() {
            errors::unbalanced_brackets();
        }

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
