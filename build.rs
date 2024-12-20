use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

fn concat_parsers(files: &[&str], out: &str) -> io::Result<()> {
    let root = "src/parser/";
    let ext = ".lalrpop";

    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}{}{}", root, out, ext))?;

    for (i, &file) in files.iter().enumerate() {
        let mut content = String::new();
        let mut f = File::open(format!("{}{}{}", root, file, ext))?;
        f.read_to_string(&mut content)?;

        if i > 0 {
            output.write_all(b"\n")?;
        }
        output.write_all(content.as_bytes())?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    concat_parsers(&["mod","exp","var","stmt","func",],"sysy")?;
    lalrpop::Configuration::new().process_file("src/parser/sysy.lalrpop").unwrap();
    Ok(())
}
