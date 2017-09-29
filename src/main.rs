#![cfg(not(test))]

extern crate libc;
extern crate rustc_serialize;
extern crate rustbox;
extern crate docopt;
extern crate iota;

use std::io::stdin;
use docopt::Docopt;
use iota::{
    Editor, Input,
    StandardMode, NormalMode, GUIMode,
    Mode, Options,
};
use rustbox::{InitOptions, RustBox, InputMode, OutputMode};
static USAGE: &'static str = "
Usage: iota [<filename>] [options]
       iota --help

Options:
    --vi                           Start Iota with vi-like modes
    --gui                          Start Iota with common graphical key bindings
    --enable-syntax-highlighting   Start Iota with syntax-highlighting enabled
    -h, --help                     Show this message.
";


#[derive(RustcDecodable, Debug)]
struct Args {
    arg_filename: Option<String>,
    flag_vi: bool,
    flag_gui: bool,
    flag_enable_syntax_highlighting: bool,
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
    let rb = match RustBox::init(InitOptions{
        buffer_stderr: stderr_is_atty,
        input_mode: InputMode::Esc,
        output_mode: OutputMode::EightBit,
    }) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    // initialise the editor mode
    let mode: Box<Mode> = if args.flag_vi {
        Box::new(NormalMode::new())
    } else if args.flag_gui {
         Box::new(GUIMode::new())
    } else {
         Box::new(StandardMode::new())
    };

    let options = Options {
        syntax_enabled: args.flag_enable_syntax_highlighting,
    };

    // start the editor
    let mut editor = Editor::new(source, mode, rb, options);
    editor.start();
}
