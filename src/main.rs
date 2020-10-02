mod errors;
mod pesc;
mod stdlib;
mod clihints;
mod tty;
mod output;

use crate::pesc::*;
use crate::clihints::*;
use crate::output::*;

use rustyline::{
    config::{
        Builder,
        EditMode,
    },
    error::ReadlineError,
    Editor,
};

fn main() {
    let mut pesc = Pesc::new();
    let output = OutputMode::auto();

    for func in stdlib::functions() {
        pesc.load(func.0, func.1, func.2);
    }

    // waitaminute, let's see if there are args we
    // can execute
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let parsed = match pesc.parse(&args[1]) {
            Ok(r) => r,
            Err(e) => {
                println!("error: {}", e);
                return;
            },
        };

        match pesc.eval(&parsed.1) {
            Ok(()) => output.format_stack(&pesc),
            Err(e) => println!("error: {}", e),
        }

        return;
    }

    // nope, display a pretty prompt & take orders
    // from stdin
    let config = Builder::new()
        .auto_add_history(true)
        .history_ignore_space(true)
        .edit_mode(EditMode::Vi)
        .build();

    let mut rl = Editor::<CommandHinter>::with_config(config);
    rl.set_helper(Some(CommandHinter::new(hints(&pesc))));

    loop {
        match rl.readline("pesc> ") {
            Ok(line) => {
                let parsed = match pesc.parse(&line) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("error: {}", e);
                        continue;
                    },
                };

                match pesc.eval(&parsed.1) {
                    Ok(()) => (),
                    Err(e) => println!("error: {}", e),
                }

                output.format_stack(&pesc);
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) =>
                println!("Use Ctrl-D to quit."),
            Err(_) => output.format_stack(&pesc),
        }
    }
}
