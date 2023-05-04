// [[file:../gosh-shell.note::7643ea86][7643ea86]]
use gut::prelude::*;
use std::path::{Path, PathBuf};
// 7643ea86 ends here

// [[file:../gosh-shell.note::f90f0bfb][f90f0bfb]]
mod helper;
// f90f0bfb ends here

// [[file:../gosh-shell.note::845cbd1e][845cbd1e]]
use rustyline::{history::FileHistory, Editor};
type MyEditor<R> = Editor<helper::MyHelper<R>, FileHistory>;

/// An shell-like REPL interpreter.
pub struct Interpreter<A> {
    prompt: String,
    history_file: Option<PathBuf>,
    action: A,
}
// 845cbd1e ends here

// [[file:../gosh-shell.note::aa47dc5f][aa47dc5f]]
impl<A: Actionable> Interpreter<A> {
    /// Interpret one line.
    fn continue_interpret_line(&mut self, line: &str) -> bool {
        if let Some(mut args) = shlex::split(line) {
            assert!(args.len() >= 1);
            args.insert(0, self.prompt.to_owned());

            match A::try_parse_from(&args) {
                // apply subcommand
                Ok(x) => match self.action.act_on(&x) {
                    Ok(exit) => {
                        if exit {
                            return false;
                        }
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                },
                // show subcommand usage
                Err(e) => println!("{:}", e),
            }
            true
        } else {
            eprintln!("Invalid quoting: {line:?}");
            false
        }
    }

    fn continue_read_eval_print<R: HelpfulCommand>(&mut self, editor: &mut MyEditor<R>) -> bool {
        match editor.readline(&self.prompt) {
            Err(rustyline::error::ReadlineError::Eof) => false,
            Ok(line) => {
                let line = line.trim();
                if !line.is_empty() {
                    let _ = editor.add_history_entry(line);
                    self.continue_interpret_line(&line)
                } else {
                    true
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }
}

fn create_readline_editor<R: HelpfulCommand>() -> Result<Editor<helper::MyHelper<R>, FileHistory>> {
    use rustyline::{ColorMode, CompletionType, Config};

    let config = Config::builder()
        .color_mode(ColorMode::Enabled)
        .completion_type(CompletionType::Fuzzy)
        .history_ignore_dups(true)?
        .history_ignore_space(true)
        .max_history_size(1000)?
        .build();

    let mut rl = Editor::with_config(config)?;
    let h = self::helper::MyHelper::new();
    rl.set_helper(Some(h));
    Ok(rl)
}
// aa47dc5f ends here

// [[file:../gosh-shell.note::360871b3][360871b3]]
impl<A: Actionable> Interpreter<A> {
    fn load_history<R: HelpfulCommand>(&mut self, editor: &mut MyEditor<R>) -> Result<()> {
        if let Some(h) = self.history_file.as_ref() {
            editor.load_history(h).context("no history")?;
        }
        Ok(())
    }

    fn save_history<R: HelpfulCommand>(&mut self, editor: &mut MyEditor<R>) -> Result<()> {
        if let Some(h) = self.history_file.as_ref() {
            editor.save_history(h).context("write history file")?;
        }
        Ok(())
    }
}
// 360871b3 ends here

// [[file:../gosh-shell.note::05b99d70][05b99d70]]
impl<A: Actionable> Interpreter<A> {
    pub fn interpret_script(&mut self, script: &str) -> Result<()> {
        let lines = script.lines().filter(|s| !s.trim().is_empty());
        for line in lines {
            debug!("Execute: {:?}", line);
            if !self.continue_interpret_line(&line) {
                break;
            }
        }

        Ok(())
    }

    pub fn interpret_script_file(&mut self, script_file: &Path) -> Result<()> {
        let s = gut::fs::read_file(script_file)?;
        self.interpret_script(&s)?;
        Ok(())
    }
}
// 05b99d70 ends here

// [[file:../gosh-shell.note::9fdf556e][9fdf556e]]
pub trait Actionable {
    type Command;

    /// Act on `cmd`
    fn act_on(&mut self, cmd: &Self::Command) -> Result<bool>;

    /// parse Command from shell line input.
    fn try_parse_from<I, T>(iter: I) -> Result<Self::Command>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone;
}

pub trait HelpfulCommand {
    fn get_subcommands() -> Vec<String>;
    fn suitable_for_path_complete(line: &str, pos: usize) -> bool;
}

impl<T: clap::CommandFactory> HelpfulCommand for T {
    fn get_subcommands() -> Vec<String> {
        let app = Self::command();
        app.get_subcommands().map(|s| s.get_name().into()).collect()
    }

    /// try to complete when current char is a path separator: foo ./
    fn suitable_for_path_complete(line: &str, pos: usize) -> bool {
        line[..pos]
            .chars()
            .last()
            .map(|x| std::path::is_separator(x))
            .unwrap_or(false)
    }
}
// 9fdf556e ends here

// [[file:../gosh-shell.note::f3bcb018][f3bcb018]]
impl<A: Actionable> Interpreter<A> {
    #[track_caller]
    pub fn new(action: A) -> Self {
        Self {
            prompt: "> ".to_string(),
            // editor: create_readline_editor::<R>().unwrap(),
            history_file: None,
            action,
        }
    }
}

impl<A: Actionable> Interpreter<A> {
    /// Set absolute path to history file for permanently storing command history.
    pub fn with_history_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        let p = path.into();
        self.history_file = Some(p);
        self
    }

    /// Set prompting string for REPL.
    pub fn with_prompt(mut self, s: &str) -> Self {
        self.prompt = s.into();
        self
    }

    /// Entry point for REPL.
    pub fn start_repl<R: HelpfulCommand>(&mut self) -> Result<()> {
        let version = env!("CARGO_PKG_VERSION");
        println!("This is the interactive parser, version {}.", version);
        println!("Enter \"help\" or \"?\" for a list of commands.");
        println!("Press Ctrl-D or enter \"quit\" or \"q\" to exit.");
        println!("");

        let mut editor = create_readline_editor::<R>()?;
        let _ = self.load_history(&mut editor);
        while self.continue_read_eval_print(&mut editor) {
            trace!("excuted one loop");
        }
        self.save_history(&mut editor)?;

        Ok(())
    }
}
// f3bcb018 ends here
