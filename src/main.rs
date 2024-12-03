mod ast;
mod codegen;
mod ir;

use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;
use std::io;
use std::fmt;

lalrpop_mod! {
    #[allow(clippy::all)]
    sysy
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        exit(-1);
    }
}

fn run() -> Result<(), Error> {
    let CommandLineArgs {
        mode,
        input,
        output,
    } = CommandLineArgs::parse()?;

    let input = read_to_string(input).map_err(Error::File)?;
    let comp_unit = sysy::CompUnitParser::new()
        .parse(&input)
        .map_err(|_| Error::Parse)?;
    let program = ir::generate_program(&comp_unit).map_err(Error::Generate)?;

    if matches!(mode, Mode::Koopa) {
        return KoopaGenerator::from_path(output)
        .map_err(Error::File)?
        .generate_on(&program)
        .map_err(Error::Io);
    } else {
        return codegen::generate_asm(&program, &output).map_err(Error::Io);
    }
}

enum Error {
    InvalidArgs,
    File(io::Error),
    Parse,
    Generate(ir::Error),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
        Self::InvalidArgs => write!(
            f,
            r#"Usage: kira MODE INPUT -o OUTPUT

Options:
MODE:   can be `-koopa`, `-riscv`
INPUT:  the input SysY source file
OUTPUT: the output file"#
    ),
        Self::File(err) => write!(f, "invalid input SysY file: {}", err),
        Self::Parse => write!(f, "error occurred while parsing"),
        Self::Generate(err) => write!(f, "{}", err),
        Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

struct CommandLineArgs {
    mode: Mode,
    input: String,
    output: String,
}

impl CommandLineArgs {
    fn parse() -> Result<Self, Error> {
        let mut args = args();
        args.next();
        match (args.next(), args.next(), args.next(), args.next()) {
            (Some(m), Some(input), Some(o), Some(output)) if o == "-o" => {
                let mode = match m.as_str() {
                "-koopa" => Mode::Koopa,
                "-riscv" => Mode::Riscv,
                _ => return Err(Error::InvalidArgs),
                };
                Ok(Self {
                mode,
                input,
                output,
                })
            }
            _ => Err(Error::InvalidArgs),
        }
    }
}

enum Mode {
    Koopa,
    Riscv,
}

