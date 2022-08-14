#![allow(
    clippy::let_underscore_untyped,
    clippy::match_bool,
    clippy::uninlined_format_args
)]

mod serve;

use clap::{Parser as ClapParser, Subcommand as ClapSubcommand};
use oqueue::{Color::Red, Sequencer};
use rust_quiz::error::Error;
use rust_quiz::render;
use std::io::{self, Write};
use std::process;

const HELP: &str = "\
{about}
{author}

{usage-heading} {usage}

{all-args}\
";

#[derive(ClapParser, Debug)]
#[command(
    about = "Rust Quiz",
    version,
    author,
    help_template = HELP,
    disable_help_subcommand = true,
)]
struct Opt {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(ClapSubcommand, Debug)]
enum Subcommand {
    /// Serve website over http at localhost:8000
    Serve,
}

fn report(result: Result<(), Error>) {
    if let Err(err) = result {
        let task = Sequencer::stderr().begin();
        task.bold_color(Red);
        write!(task, "ERROR");
        task.bold();
        writeln!(task, ": {}", err);
        task.reset_color();
        process::exit(1);
    }
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    report(render::main());

    match opt.subcommand {
        None => {}
        Some(Subcommand::Serve) => {
            let _ = writeln!(io::stderr());
            report(serve::main().await);
        }
    }
}

#[test]
fn test_cli() {
    <Opt as clap::CommandFactory>::command().debug_assert();
}
