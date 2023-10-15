use std::{io::Write, process::exit};

use cool_cpu_assembler::{assembler::remove_labels, grammar};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    input: String,
    #[arg(short, default_value_t=String::from("out.bin"))]
    output: String
}


fn main() {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.input).unwrap();
    let ast = remove_labels(grammar::AsmParser::new().parse(&input).unwrap_or_else(|e| {
        println!("{e}");
        exit(-1);
    }));
    let mut output = String::new();
    for word in cool_cpu_assembler::assembler::assemble(ast) {
        output.push_str(&format!("{:02x}\n", word));
    }
    std::fs::File::create(args.output)
        .unwrap()
        .write_all(&output.as_bytes())
        .unwrap();
}
