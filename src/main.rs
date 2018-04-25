#![cfg(not(test))]

extern crate iota;
extern crate libc;
extern crate rustbox;

use iota::{Editor, Input};
use rustbox::{InitOptions, InputMode, OutputMode, RustBox};
use std::io::stdin;
use std::env;

fn is_atty(fileno: libc::c_int) -> bool {
    // FIXME: find a way to do this without unsafe
    //        std::io doesn't allow for this, currently
    unsafe { libc::isatty(fileno) != 0 }
}

fn main() {
    let stdin_is_atty = is_atty(libc::STDIN_FILENO);
    let stderr_is_atty = is_atty(libc::STDERR_FILENO);

    // editor source - either a filename or stdin
    let source = if stdin_is_atty {
        Input::Filename(env::args().nth(1))
    } else {
        Input::Stdin(stdin())
    };

    // initialise rustbox
    let rb = match RustBox::init(InitOptions {
        buffer_stderr: stderr_is_atty,
        input_mode: InputMode::Esc,
        output_mode: OutputMode::EightBit,
    }) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    // start the editor
    let mut editor = Editor::new(source, rb);
    editor.start();
}
