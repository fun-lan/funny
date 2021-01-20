use super::def::Cmd;
use clap::ArgMatches;
use std::fs;

pub struct RunCmd {}

impl Cmd for RunCmd {
    fn name(&self) -> String {
        String::from("run")
    }

    fn exec(&self, args: &ArgMatches<'static>) -> Result<(), idioma::Error> {
        let source_file = args
            .value_of("SOURCE")
            .expect("Source file must be specified");
        let source_code = read_source_file(source_file)?;
        println!("Source code from '{}':\n{}", source_file, source_code);
        Ok(())
    }
}

fn read_source_file(path: &str) -> Result<String, idioma::Error> {
    let data = idioma::into(fs::read_to_string(path))?;
    Ok(data)
}
