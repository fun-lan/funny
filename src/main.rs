#[macro_use]
extern crate clap;
extern crate idioma;

use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let app = cli();

    for cmd in SUBCOMMANDS.iter() {
        if matched(cmd, &app) {
            return;
        }
    }

    idioma::error("You didn't specify a sub-command. I'm a bit lost...").display();
    idioma::info("Run 'funny --help' to see usage hints.").exit_with();
}

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

struct Command {
    name: &'static str,
    func: fn(&ArgMatches<'static>),
}

const SUBCOMMANDS: [Command; 1] = [Command {
    name: "run",
    func: run,
}];

fn matched(cmd: &Command, app: &ArgMatches<'static>) -> bool {
    if let Some(matches) = app.subcommand_matches(cmd.name) {
        (cmd.func)(matches);
        true
    } else {
        false
    }
}

fn run(cmd: &ArgMatches<'static>) {
    let src = cmd
        .value_of("SOURCE")
        .expect("source file must be specified");
    println!("Running: {} ...", src);
}
