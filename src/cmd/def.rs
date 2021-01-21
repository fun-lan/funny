use clap::ArgMatches;

/// Cmd trait represents the structure of a Funny sub-command.
pub trait Cmd {
    fn name(&self) -> String;
    fn exec(&self, args: &ArgMatches<'static>) -> Result<(), idioma::Error>;
}
