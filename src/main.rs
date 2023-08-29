use std::{io::Write, process::exit};

use assembler::{assembler::remove_labels, grammar};

fn main() {
    let input = std::fs::read_to_string("in.s").unwrap();
    let ast = remove_labels(grammar::AsmParser::new().parse(&input).unwrap_or_else(|e| {
        println!("{e}");
        exit(-1);
    }));
    std::fs::File::create("out.bin")
        .unwrap()
        .write_all(&assembler::assembler::assemble(ast))
        .unwrap();
}
