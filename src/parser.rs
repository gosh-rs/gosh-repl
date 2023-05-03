// [[file:../gosh-shell.note::70d3dbdb][70d3dbdb]]
use gut::prelude::*;
// 70d3dbdb ends here

// [[file:../gosh-shell.note::724d9a95][724d9a95]]
use gut::cli::*;

#[derive(Parser, Debug)]
#[clap(disable_help_subcommand = true)]
pub enum Cmd {
    /// Quit shell.
    #[clap(name = "quit", alias = "q", alias = "exit")]
    Quit {},

    /// Show available commands.
    #[clap(name = "help", alias = "h", alias = "?")]
    Help {},

    /// Load file from `path` for processing.
    ///
    /// WARNING: load very large file may lead to out of memory error.
    #[clap(name = "load")]
    Load {
        #[clap(name = "FILENAME")]
        path: String,
    },

    /// Move cursor to line `line_num`
    #[clap(name = "goto-line")]
    GotoLine {
        #[clap(name = "LINE-NUMBER")]
        line_num: usize,

        /// Specify line range relative to current line (cursor position)
        #[clap(long)]
        relative: bool,
    },

    /// Display a line of `text`
    #[clap(name = "println")]
    Println { text: String },

    /// Move cursor the line matching search `pattern`.
    #[clap(name = "search-forward")]
    SearchForward {
        #[clap(name = "PATTERN")]
        pattern: String,
    },

    /// Select a list of lines: 1-3 or 4
    #[clap(name = "select-lines")]
    SelectLines {
        #[clap(name = "LINE-SPECS")]
        specs: String,

        /// Specify line range relative to current line (cursor position)
        #[clap(long)]
        relative: bool,
    },

    /// Print selected lines
    #[clap(name = "print-selection")]
    PrintSelection {
        /// Print the text within selected columns.
        #[clap(long)]
        columns: Option<String>,

        /// Writes selection into external command through a pipe.
        #[clap(long)]
        pipe: Option<String>,
    },
}
// 724d9a95 ends here

// [[file:../gosh-shell.note::a252f98f][a252f98f]]
#[derive(Debug, Default, Clone)]
struct Action {
    // glance: Option<Glance>,
}

impl crate::repl::Actionable for Action {
    type Command = Cmd;

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
                app.print_help();
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

// [[file:../gosh-shell.note::f8cc322b][f8cc322b]]
impl crate::repl::HelpfulCommand for Cmd {
    fn get_subcommands() -> Vec<String> {
        let app = Cmd::command();
        app.get_subcommands().map(|s| s.get_name().into()).collect()
    }

    fn suitable_for_path_complete(line: &str) -> bool {
        line.trim().starts_with("load")
    }
}
// f8cc322b ends here

// [[file:../gosh-shell.note::f12dda7e][f12dda7e]]
pub mod cli {
    use super::Action;
    use crate::repl::Interpreter;
    use gut::cli::*;
    use gut::prelude::*;
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
                    use std::io::{self, Read};

                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer)?;
                    Interpreter::new(action).interpret_script(&buffer)?;
                }
            } else {
                Interpreter::new(action).with_prompt("gosh> ").start_repl()?;
            }

            Ok(())
        }
    }
}
// f12dda7e ends here
