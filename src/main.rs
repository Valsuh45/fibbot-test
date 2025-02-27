use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output = File::create("/github/workspace/output.txt").expect("Failed to create output file");

    if args.len() != 3 {
        writeln!(output, "Enter exactly two arguments. Invalid input!!!").unwrap();
        println!("Enter exactly two arguments. Invalid input!!!");
    } else {
        let argument_1 = &args[1];
        let argument_2 = &args[2];

        writeln!(output, "{}", argument_1).unwrap();
        writeln!(output, "{}", argument_2).unwrap();

        println!("Hello, world!");
        println!("{}", argument_1);
        println!("{}", argument_2);
    }
}
