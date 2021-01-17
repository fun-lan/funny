extern crate clap;
extern crate idioma;
use std::fs;

use clap::ArgMatches;

/// Each sub-command must have an `exec` function with this signature.
pub fn exec(cmd: &ArgMatches<'static>) -> Result<(), idioma::Error> {
    let source_file = cmd
        .value_of("SOURCE")
        .expect("Source file must be specified");
    let source_code = read_source_file(source_file)?;
    println!("Source code from '{}':\n{}", source_file, source_code);
    Ok(())
}

fn read_source_file(path: &str) -> Result<String, idioma::Error> {
    let data = idioma::into(fs::read_to_string(path))?;
    Ok(data)
}
