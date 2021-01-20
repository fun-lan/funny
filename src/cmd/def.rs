use clap::ArgMatches;

pub trait Cmd {
    fn name(&self) -> String;
    fn exec(&self, args: &ArgMatches<'static>) -> Result<(), idioma::Error>;
}
