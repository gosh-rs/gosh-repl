// [[file:../gosh-shell.note::83dd8964][83dd8964]]
// #![deny(warnings)]
#![deny(clippy::all)]
// 83dd8964 ends here

// [[file:../gosh-shell.note::88743546][88743546]]
mod parser;
mod repl;
// mod skim;
// 88743546 ends here

// [[file:../gosh-shell.note::*docs][docs:1]]
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

    // export_doc!(codec);
}
// docs:1 ends here
