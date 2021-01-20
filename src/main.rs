#[macro_use]
extern crate clap;
extern crate idioma;

mod ast;
mod run;

use clap::{App, Arg, ArgMatches, SubCommand};

#[allow(unused)]
// The `allow(unused)` directive is here because the compiler can't properly understand the flow of
// control within `exit_if_error` and gives a warning.
fn main() {
    let app = cli();

    for cmd in SUBCOMMANDS.iter() {
        if let Some(result) = matched(cmd, &app) {
            idioma::exit_if_error(result);
            return;
        }
    }

    idioma::error("You didn't specify a sub-command. I'm a bit lost...").print();
    idioma::info("Run 'funny --help' to see usage hints.").exit(1);
}

/// CLI setup that parses our command line arguments for us.
fn cli() -> ArgMatches<'static> {
    App::new("Funny Programming Language")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Funny tools suite")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a Funny program")
                .arg(
                    Arg::with_name("SOURCE")
                        .required(true)
                        .index(1)
                        .help("Source file path"),
                ),
        )
        .get_matches()
}

/// Command only has a name and an `exec` function that takes `ArgMatches` and returns a `Result`.
/// We essentially treat sub-commands as independent sub-main functions to which the arguments are
/// dispatched.
struct Command {
    name: &'static str,
    exec: fn(&ArgMatches<'static>) -> Result<(), idioma::Error>,
}

/// This is just an array of all available sub-commands.
const SUBCOMMANDS: [Command; 1] = [Command {
    name: "run",
    exec: run::exec,
}];

/// This function takes a `Command` and `ArgMatches` and tries to match `Command`'s name to a
/// sub-command. If the match was found, it calls `Command`'s function and returns its `Result`
/// wrapped in `Option`.
///
/// The additional `Option` wrapper is there to distinguish between no match and failed execution.
fn matched(cmd: &Command, app: &ArgMatches<'static>) -> Option<Result<(), idioma::Error>> {
    if let Some(matches) = app.subcommand_matches(cmd.name) {
        Some((cmd.exec)(matches))
    } else {
        None
    }
}
