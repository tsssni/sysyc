mod ast;
mod ir;

use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;
use std::io;
use std::fmt;

lalrpop_mod!(sysy);

enum Error {
    InvalidArgs,
    Generate(ir::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidArgs => write!(f, "usage: sysyc -i INPUT -o OUTPUT"),
            Self::Generate(err) => write!(f, "{}", err),
            Self::Io(err) => write!(f, "I/O error: {}", err), 
        }
    }
}

struct Args {
    input: String,
    output: String,
}

impl Args {
    fn parse() -> Result<Self, Error> {
        let mut args = args();
        args.next();

        match (args.next(), args.next(), args.next(), args.next()) {
            (Some(i), Some(input), Some(o), Some(output)) => {
                if i == "-i" && o == "-o" {
                    return Ok(Self {input, output});
                }
                Err(Error::InvalidArgs)
            }
            _ => Err(Error::InvalidArgs)
        }
    }
}

fn run() -> Result<(), Error> {
    let Args { input, output } = Args::parse()?;

    let input = read_to_string(input).map_err(Error::Io)?;

    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    let program = ir::generate_program(&ast).map_err(Error::Generate)?;

    KoopaGenerator::from_path(output)
        .map_err(Error::Io)?
        .generate_on(&program)
        .map_err(Error::Io)?;
    println!("{:#?}", ast);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        exit(1);
    }
}
