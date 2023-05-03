// [[file:../../gosh-shell.note::e0fe07d2][e0fe07d2]]
use super::*;
use std::marker::PhantomData;

use rustyline::completion::{FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::Context;
use rustyline_derive::{Completer, Helper, Highlighter, Validator};

#[derive(Helper, Highlighter, Validator)]
pub struct MyHelper<R: HelpfulCommand> {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    colored_prompt: String,
    // for advanced completion
    _R: PhantomData<R>,
}

impl<R: HelpfulCommand> rustyline::completion::Completer for MyHelper<R> {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        if R::suitable_for_path_complete(line, pos) {
            self.completer.complete(line, pos, ctx)
        } else {
            let commands = R::get_subcommands();
            let pairs = commands
                .into_iter()
                .filter_map(|x| {
                    if x.starts_with(line) {
                        new_candidate(&x).into()
                    } else {
                        None
                    }
                })
                .collect();
            Ok((0, pairs))
        }
    }
}

impl<R: HelpfulCommand> MyHelper<R> {
    pub fn new() -> Self {
        Self {
            completer: FilenameCompleter::new(),
            colored_prompt: "".to_owned(),
            _R: PhantomData,
        }
    }
}

impl<R: HelpfulCommand> rustyline::hint::Hinter for MyHelper<R> {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        None
    }
}

fn new_candidate(x: &str) -> Pair {
    Pair {
        display: x.into(),
        replacement: x.into(),
    }
}
// e0fe07d2 ends here
