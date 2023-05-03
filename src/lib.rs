// [[file:../gosh-shell.note::83dd8964][83dd8964]]
// #![deny(warnings)]
#![deny(clippy::all)]
// 83dd8964 ends here

// [[file:../gosh-shell.note::88743546][88743546]]
mod parser;
mod repl;
// mod skim;
// 88743546 ends here

// [[file:../gosh-shell.note::67e2867b][67e2867b]]
#[cfg(feature = "adhoc")]
/// Docs for local mods
pub mod docs {
    macro_rules! export_doc {
        ($l:ident) => {
            pub mod $l {
                pub use crate::$l::*;
            }
        };
    }

    export_doc!(repl);
    export_doc!(parser);
}
// 67e2867b ends here

// [[file:../gosh-shell.note::9cc4dec6][9cc4dec6]]
pub use parser::cli::*;

pub use repl::Interpreter;
// 9cc4dec6 ends here
