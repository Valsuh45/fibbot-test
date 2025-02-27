use std::env;
use std::fs::File;
use std::io::{Write, BufWriter};

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_path = "/output.txt";
    let file = File::create(output_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    if args.len() != 3 {
        writeln!(writer, "Error: Enter exactly two arguments. Invalid input!").unwrap();
        println!("Error: Enter exactly two arguments. Invalid input!");
    } else {
        let argument_1 = &args[1];
        let argument_2 = &args[2];

        writeln!(writer, "{}", argument_1).unwrap();
        writeln!(writer, "{}", argument_2).unwrap();

        println!("Successfully wrote to {}", output_path);
    }
}
