use crate::parser::Node;

pub struct Codegen {
    loop_id: usize,
}

impl Codegen {
    fn new() -> Codegen {
        Codegen { loop_id: 0 }
    }

    pub fn generate(nodes: &Vec<Node>) -> String {
        let mut codegen = Codegen::new();
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
            codegen._generate(nodes)
        )
    }

    fn _generate(&mut self, nodes: &Vec<Node>) -> String {
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

                    asm += &self._generate(&nodes);
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
