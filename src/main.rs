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
    let mut file = std::fs::File::create(args.output).unwrap();
    output += "always_comb begin\ncase(read_pos)\n";
    for (pos, word) in cool_cpu_assembler::assembler::assemble(ast).into_iter().enumerate() {
        output += &format!("{}: data_inner = 8'h{:02x};\n", pos, word);
        file.write_all(&word.to_le_bytes()).unwrap();
    }
    output += "default: data_inner = 8'h00;\nendcase\nend";
}
