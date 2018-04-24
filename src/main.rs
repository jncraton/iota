#![cfg(not(test))]

extern crate docopt;
extern crate iota;
extern crate libc;
extern crate rustbox;
extern crate rustc_serialize;

use docopt::Docopt;
use iota::{Editor, Input};
use rustbox::{InitOptions, InputMode, OutputMode, RustBox};
use std::io::stdin;
static USAGE: &'static str = "
Usage: iota [<filename>] [options]
       iota --help

Options:
    -h, --help                     Show this message.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_filename: Option<String>,
    flag_help: bool,
}

fn is_atty(fileno: libc::c_int) -> bool {
    // FIXME: find a way to do this without unsafe
    //        std::io doesn't allow for this, currently
    unsafe { libc::isatty(fileno) != 0 }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let stdin_is_atty = is_atty(libc::STDIN_FILENO);
    let stderr_is_atty = is_atty(libc::STDERR_FILENO);

    // editor source - either a filename or stdin
    let source = if stdin_is_atty {
        Input::Filename(args.arg_filename)
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
