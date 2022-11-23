use std::io::{BufReader, BufWriter, BufRead, Write};
use std::fs::File;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "levenshtein Distance")]
#[command(author = "TechHara")]
#[command(version = "1.0")]
#[command(about = "Calculates levenshtein distance between two strings", long_about = None)]
struct Cli {
    #[arg(value_enum, long, default_value_t = Mode::Char)]
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
    Byte,
    Char,
    Word,
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

    let ifs = BufReader::new(File::open(input)?);
    let mut ofs = BufWriter::new(File::create(&output)?);
    let runner = get_runner(cli.mode);
    
    for (linenum, line) in ifs.lines().enumerate() {
        let line = line?;
        let strings: Vec<&str> = line.split(cli.sep).collect();
        if strings.len() != 2 {
            eprintln!("Skipping invalid line #{}: {}", linenum, strings.join(&cli.sep.to_string()));
            continue;
        }

        let result = runner(strings[0], strings[1]);
        writeln!(ofs, "{}", result)?;
    }

    Ok(())
}

fn get_runner(mode: Mode) -> fn(&str, &str) -> f64 {
    match mode {
        Mode::Byte => |x: &str, y: &str| {
            let x = x.as_bytes();
            let y = y.as_bytes();
            levenshtein_distance(x, y) as f64 / x.len().max(y.len()) as f64
        },
        Mode::Char => |x: &str, y: &str| {
            let x: Vec<char> = x.chars().collect();
            let y: Vec<char> = y.chars().collect();
            levenshtein_distance(&x, &y) as f64 / x.len().max(y.len()) as f64
        },
        Mode::Word => |x: &str, y: &str| {
            let x: Vec<&str> = x.split_whitespace().collect();
            let y: Vec<&str> = y.split_whitespace().collect();
            levenshtein_distance(&x, &y) as f64 / x.len().max(y.len()) as f64
        },
    }
}

fn levenshtein_distance<T>(x: &[T], y: &[T]) -> usize 
where T: std::cmp::Eq {
    let nx = x.len();
    let ny = y.len();
    let mut memo = vec![None; nx*ny];
    return levenshtein_distance_helper(x, y, &mut memo, ny);
}

fn levenshtein_distance_helper<T>(x: &[T], y: &[T], memo: &mut Vec<Option<usize>>, ny: usize) -> usize
where T: std::cmp::Eq {
    if x.len() == 0 || y.len() == 0 {
        return x.len().max(y.len());
    } 
    let idx = (x.len() - 1) * ny + y.len() - 1;
    memo[idx] = match memo[idx] {
        Some(val) => { return val; },
        _ => match x.last().unwrap() == y.last().unwrap() {
            true => Some(levenshtein_distance_helper(&x[..x.len()-1], &y[..y.len()-1], memo, ny)),
            false => {
                let insert = levenshtein_distance_helper(&x[..x.len() - 1], y, memo, ny);
                let delete = levenshtein_distance_helper(x, &y[..y.len() - 1], memo, ny);
                let replace = levenshtein_distance_helper(&x[..x.len() - 1], &y[..y.len() - 1], memo, ny);

                Some(insert.min(delete).min(replace) + 1)
            }
        },
    };

    memo[idx].unwrap()
}