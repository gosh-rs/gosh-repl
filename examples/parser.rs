// [[file:../gosh-shell.note::70d3dbdb][70d3dbdb]]
#![deny(warnings)]

use gosh_repl::{Actionable, Interpreter};

use gut::cli::*;
use gut::prelude::*;
// 70d3dbdb ends here

// [[file:../gosh-shell.note::724d9a95][724d9a95]]
#[derive(Parser, Debug)]
#[clap(disable_help_subcommand = true)]
enum Cmd {
    /// Quit shell.
    #[command(name = "quit", alias = "q", alias = "exit")]
    Quit {},

    /// Show available commands.
    #[command(name = "help", alias = "h", alias = "?")]
    Help {},

    /// Load file from `path` for processing.
    #[command(name = "load")]
    Load {
        #[clap(name = "FILENAME")]
        path: String,
    },
}
// 724d9a95 ends here

// [[file:../gosh-shell.note::a252f98f][a252f98f]]
#[derive(Debug, Default, Clone)]
struct Action {
    // state var during REPL
    _state: Option<Vec<String>>,
}

impl Actionable for Action {
    type Command = Cmd;

    /// parse REPL commands from shell line input using clap
    fn try_parse_from<I, T>(iter: I) -> Result<Self::Command>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let r = Cmd::try_parse_from(iter)?;
        Ok(r)
    }

    /// Take action on REPL commands. Return Ok(true) will exit shell
    /// loop.
    fn act_on(&mut self, cmd: &Cmd) -> Result<bool> {
        match cmd {
            Cmd::Quit {} => return Ok(true),

            Cmd::Help {} => {
                let mut app = Cmd::command();
                if let Err(err) = app.print_help() {
                    eprintln!("clap error: {err:?}");
                }
                println!("");
            }

            o => {
                eprintln!("{:?}: not implemented yet!", o);
            }
        }

        Ok(false)
    }
}
// a252f98f ends here

// [[file:../gosh-shell.note::f12dda7e][f12dda7e]]
mod cli {
    #![deny(warnings)]

    use super::*;
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    pub struct ReplCli {
        /// Execute REPL script
        #[clap(short = 'x')]
        script_file: Option<PathBuf>,

        #[clap(flatten)]
        verbose: Verbosity,
    }

    impl ReplCli {
        pub fn enter_main() -> Result<()> {
            let args: Vec<String> = std::env::args().collect();

            let action = Action::default();
            // enter shell mode or subcommands mode
            if args.len() > 1 {
                let args = Self::parse();
                args.verbose.setup_logger();

                if let Some(script_file) = &args.script_file {
                    info!("Execute script file: {:?}", script_file);
                    Interpreter::new(action).interpret_script_file(script_file)?;
                } else {
                    info!("Reading batch script from stdin ..");
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer)?;
                    Interpreter::new(action).interpret_script(&buffer)?;
                }
            } else {
                Interpreter::new(action).with_prompt("gosh> ").start_repl::<Cmd>()?;
            }

            Ok(())
        }
    }
}
// f12dda7e ends here

// [[file:../gosh-shell.note::e817ae85][e817ae85]]
fn main() -> Result<()> {
    cli::ReplCli::enter_main()?;
    Ok(())
}
// e817ae85 ends here
