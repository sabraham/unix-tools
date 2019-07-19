use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

struct Config {
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
}

struct Output {
    bytes: u32,
    lines: u32,
    chars: u32,
    words: u32,
    filename: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            bytes: true,
            lines: true,
            chars: false,
            words: true,
        }
    }
}

fn process<T: io::Read>(filename: Option<String>,
    reader: BufReader<T>,
) -> Result<Output, io::Error> {
    let (mut bytes, mut lines, mut words) = (0, 0, 0);
    let mut whitespace = true;
    for byte in reader.bytes() {
        bytes += 1;
        match byte {
            Ok(b'\n') => {
                whitespace = true;
                lines = lines + 1;
            }
            Ok(b'\t') | Ok(b' ') => {
                whitespace = true;
            }
            Err(x) => return Err(x),
            _ => {
                if whitespace {
                    whitespace = false;
                    words = words + 1;
                }
                ()
            }
        };
    }
    Ok(Output {
        bytes,
        lines,
        chars:0,
        words,
        filename,
    })
}

fn display(config: &Config, output: &Output) -> String {
    let mut res = String::new();
    if config.lines {
        res.push_str(&format!("{:>8}", output.lines));
    }
    if config.words {
        res.push_str(&format!("{:>8}", output.words));
    }
    if config.bytes {
        res.push_str(&format!("{:>8}", output.bytes));
    }
    if config.chars {
        res.push_str(&format!("{:>8}", output.chars));
    }

    match &output.filename {
        Some(x) => res.push_str(&format!(" {:<}", x)),
        _ => (),
    }
    res
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::new();
    let mut files = None;
    for (i, arg) in args.iter().skip(1).enumerate() {
        let mut it = arg.chars();
        if it.next().unwrap() == '-' {
            config.bytes = false;
            config.lines = false;
            config.chars = false;
            config.words = false;
            while let Some(ch) = it.next() {
                match ch {
                    'c' => config.bytes = true,
                    'l' => config.lines = true,
                    'm' => {
                        config.chars = true;
                        eprintln!("-m is not implemented yet.");
                    }
                    'w' => config.words = true,
                    flag => {
                        eprintln!("Unsupported flag: {}", flag);
                    }
                }
            }
            continue;
        }
        files = Some((&args[i + 1..]).into_iter());
        break;
    }
    let results = match files {
        None => {
            let reader = BufReader::new(io::stdin());
            let output = process(None, reader);
            vec![output]
        }
        Some(x) => x
            .map(|x| {
                let f = File::open(x).unwrap();
                let reader = BufReader::new(f);
                process(Some(x.to_string()), reader)
            })
            .collect(),
    };

    for result in results {
        match result {
            Ok(y) => println!("{}", display(&config, &y)),
            Err(_) => (),
        }
    }
    Ok(())
}
