use std::io::{BufReader, BufWriter, BufRead, Write};
use std::fs::File;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "levenshtein Distance")]
#[command(author = "TechHara")]
#[command(version = "1.0")]
#[command(about = "Calculates levenshtein distance between two strings", long_about = None)]
struct Cli {
    #[arg(value_enum, long, default_value_t = Metric::Lev)]
    metric: Metric,
    #[arg(value_enum, long, default_value_t = Atom::Char)]
    atom: Atom,
    /// separator between two strings
    #[arg(long, default_value_t = '\t')]
    sep: char,
    /// input file
    input: Option<String>,
    /// output file
    output: Option<String>,
}

#[derive(Clone, ValueEnum)]
enum Atom {
    Byte,
    Char,
    Word,
}

#[derive(Clone, ValueEnum)]
enum Metric {
    /// Levenshtein
    Lev,
    /// optimal string alignment
    OSA,
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
    let runner = get_runner(cli.atom, cli.metric);
    
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

fn get_runner(atom: Atom, metric: Metric) -> impl Fn(&str, &str) -> f64 {
    match metric {
        Metric::Lev => match atom {
            Atom::Byte => |x: &str, y: &str| {
                let x = x.as_bytes();
                let y = y.as_bytes();
                levenshtein_distance(x, y) as f64 / x.len().max(y.len()) as f64
            },
            Atom::Char => |x: &str, y: &str| {
                let x: Vec<char> = x.chars().collect();
                let y: Vec<char> = y.chars().collect();
                levenshtein_distance(&x, &y) as f64 / x.len().max(y.len()) as f64
            },
            Atom::Word => |x: &str, y: &str| {
                let x: Vec<&str> = x.split_whitespace().collect();
                let y: Vec<&str> = y.split_whitespace().collect();
                levenshtein_distance(&x, &y) as f64 / x.len().max(y.len()) as f64
            },
        },
        Metric::OSA => match atom {
            Atom::Byte => |x: &str, y: &str| {
                let x = x.as_bytes();
                let y = y.as_bytes();
                osa_distance(x, y) as f64 / x.len().max(y.len()) as f64
            },
            Atom::Char => |x: &str, y: &str| {
                let x: Vec<char> = x.chars().collect();
                let y: Vec<char> = y.chars().collect();
                osa_distance(&x, &y) as f64 / x.len().max(y.len()) as f64
            },
            Atom::Word => |x: &str, y: &str| {
                let x: Vec<&str> = x.split_whitespace().collect();
                let y: Vec<&str> = y.split_whitespace().collect();
                osa_distance(&x, &y) as f64 / x.len().max(y.len()) as f64
            },
        },
    }
}

fn levenshtein_distance<T>(x: &[T], y: &[T]) -> usize 
where T: std::cmp::Eq {
    let nx = x.len();
    let ny = y.len();
    let mut memo = vec![usize::MAX; nx*ny];
    levenshtein_distance_helper(x, y, &mut memo, ny)
}

fn levenshtein_distance_helper<T>(x: &[T], y: &[T], memo: &mut Vec<usize>, ny: usize) -> usize
where T: std::cmp::Eq {
    if x.len() == 0 || y.len() == 0 {
        return x.len().max(y.len());
    } 
    let idx = (x.len() - 1) * ny + y.len() - 1;
    memo[idx] = match memo[idx] == usize::MAX {
        false => { return memo[idx]; },
        true => match x.last().unwrap() == y.last().unwrap() {
            true => levenshtein_distance_helper(&x[..x.len()-1], &y[..y.len()-1], memo, ny),
            false => {
                let insert = levenshtein_distance_helper(&x[..x.len() - 1], y, memo, ny);
                let delete = levenshtein_distance_helper(x, &y[..y.len() - 1], memo, ny);
                let replace = levenshtein_distance_helper(&x[..x.len() - 1], &y[..y.len() - 1], memo, ny);

                insert.min(delete).min(replace) + 1
            }
        },
    };

    memo[idx]
}

fn osa_distance<T>(x: &[T], y: &[T]) -> usize 
where T: std::cmp::Eq {
    let nx = x.len();
    let ny = y.len();
    let mut memo = vec![usize::MAX; nx*ny];
    osa_distance_helper(x, y, &mut memo, ny)
}

fn osa_distance_helper<T>(x: &[T], y: &[T], memo: &mut Vec<usize>, ny: usize) -> usize
where T: std::cmp::Eq {
    if x.len() == 0 || y.len() == 0 {
        return x.len().max(y.len());
    } 
    let idx = (x.len() - 1) * ny + y.len() - 1;
    memo[idx] = match memo[idx] == usize::MAX {
        false => { return memo[idx]; },
        _ => match x.last().unwrap() == y.last().unwrap() {
            true => osa_distance_helper(&x[..x.len()-1], &y[..y.len()-1], memo, ny),
            false => {
                let insert = osa_distance_helper(&x[..x.len() - 1], y, memo, ny);
                let delete = osa_distance_helper(x, &y[..y.len() - 1], memo, ny);
                let replace = osa_distance_helper(&x[..x.len() - 1], &y[..y.len() - 1], memo, ny);
                let transpose = match x.len() >= 2 && y.len() >= 2 && x[x.len()-1] == y[y.len()-2] && x[x.len()-2] == y[y.len()-1] {
                    true => osa_distance_helper(&x[..x.len()-2], &y[..y.len()-2], memo, ny),
                    false => insert,
                };

                insert.min(delete).min(replace).min(transpose) + 1
            }
        },
    };

    memo[idx]
}
