use std::io::{BufReader, BufWriter, BufRead, Write};
use std::fs::File;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "Levenstein Distance")]
#[command(author = "TechHara")]
#[command(version = "1.0")]
#[command(about = "Calculates Levenstein distance between two strings", long_about = None)]
struct Cli {
    #[arg(value_enum, long, default_value_t = Mode::char)]
    mode: Mode,
    /// separator between two strings
    #[arg(long, default_value_t = '\t')]
    sep: char,
    /// input file
    input: Option<String>,
    /// output file
    output: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    byte,
    char,
    word,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let input = cli.input.unwrap_or("/dev/stdin".to_string());
    let input = match "-" == input {
        true => "/dev/stdin".to_string(),
        false => input,
    };

    let output = cli.output.unwrap_or("/dev/stdout".to_string());
    let output = match "-" == output {
        true => "/dev/stdout".to_string(),
        false => output,
    };

    eprintln!("input: {}\toutput: {}", input, output);
    let ifs = BufReader::new(File::open(input)?);
    let mut ofs = BufWriter::new(File::create(&output)?);

    for (linenum, line) in ifs.lines().enumerate() {
        let line = line?;
        let strings: Vec::<&str> = line.split(cli.sep).collect();
        if strings.len() != 2 {
            eprintln!("Skipping invalid line #{}: {}", linenum, strings.join(&cli.sep.to_string()));
            continue;
        }
        writeln!(ofs, "{}", "");
    }

    Ok(())
}