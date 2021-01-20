#[macro_use]
extern crate clap;
extern crate colored;
extern crate idioma;

mod ast;
mod cmd;

use clap::{App, Arg, ArgMatches, SubCommand};
use cmd::{def::Cmd, run::RunCmd};
use colored::Colorize;

fn main() {
    let commands = vec![Box::new(RunCmd {}) as Box<dyn Cmd>];

    let app = cli();

    for cmd in commands.iter() {
        // The `allow(unused)` directive is here because the compiler can't properly understand the
        // flow of control within `exit_if_error` and gives a warning.
        #[allow(unused)]
        if let Some(result) = matched(cmd, &app) {
            idioma::exit_if_error(result);
            return;
        }
    }

    subcommand_not_specified();
}

/// CLI setup that parses our command line arguments for us.
fn cli() -> ArgMatches<'static> {
    App::new("Funny Programming Language")
        .version(crate_version!())
        .author(crate_authors!())
        .about("CLI tools suite for the Funny Programming Language")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a Funny program")
                .arg(
                    Arg::with_name("SOURCE")
                        .required(true)
                        .index(1)
                        .help("Path to source file"),
                ),
        )
        .get_matches()
}

/// This function takes a `cmd::Cmd` and `ArgMatches` and tries to match `Cmd`'s name to a
/// sub-command. If the match was found, it calls `Cmd`'s `exec` method and returns its `Result`
/// wrapped in `Option`.
///
/// The additional `Option` wrapper is there to distinguish between no match and failed execution.
fn matched(cmd: &Box<dyn Cmd>, app: &ArgMatches<'static>) -> Option<Result<(), idioma::Error>> {
    if let Some(matches) = app.subcommand_matches(cmd.name()) {
        Some(cmd.exec(matches))
    } else {
        None
    }
}

fn subcommand_not_specified() {
    idioma::error("You didn't specify a sub-command. I'm a bit lost...").print();
    idioma::info(format!(
        "Run '{}' to see usage hints.",
        "funny --help".green()
    ))
    .exit(1);
}
