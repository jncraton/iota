use buffer::Mark;
use command::{Action, BuilderEvent, Command, Operation};
use key::Key;
use textobject::{Anchor, Kind, Offset, TextObject};

pub fn handle_key_event(key: Key) -> BuilderEvent {
    BuilderEvent::Complete(match key {
        // Editor Commands
        Key::Ctrl('q') => Command::exit_editor(),
        Key::Ctrl('s') => Command::save_buffer(),

        Key::Char(c) => Command::insert_char(c),

        // Cursor movement
        Key::Up => Command::movement(
            Offset::Backward(1, Mark::Cursor(0)),
            Kind::Line(Anchor::Same),
        ),

        Key::Down => Command::movement(
            Offset::Forward(1, Mark::Cursor(0)),
            Kind::Line(Anchor::Same),
        ),
        Key::Left => Command::movement(Offset::Backward(1, Mark::Cursor(0)), Kind::Char),
        Key::Right => Command::movement(Offset::Forward(1, Mark::Cursor(0)), Kind::Char),

        Key::CtrlRight => Command::movement(
            Offset::Forward(1, Mark::Cursor(0)),
            Kind::Word(Anchor::Start),
        ),
        Key::CtrlLeft => Command::movement(
            Offset::Backward(1, Mark::Cursor(0)),
            Kind::Word(Anchor::Start),
        ),

        Key::End => Command::movement(Offset::Forward(0, Mark::Cursor(0)), Kind::Line(Anchor::End)),
        Key::Home => Command::movement(
            Offset::Backward(0, Mark::Cursor(0)),
            Kind::Line(Anchor::Start),
        ),

        _ => Command::noop(),
    })
}
/*

        // Editing
        keymap.bind_key(Key::Tab, Command::insert_tab());
        keymap.bind_key(Key::Enter, Command::insert_char('\n'));
        keymap.bind_key(
            Key::Backspace,
            Command {
                number: 1,
                action: Action::Operation(Operation::DeleteFromMark(Mark::Cursor(0))),
                object: Some(TextObject {
                    kind: Kind::Char,
                    offset: Offset::Backward(1, Mark::Cursor(0)),
                }),
            },
        );
        keymap.bind_key(
            Key::Delete,
            Command {
                number: 1,
                action: Action::Operation(Operation::DeleteFromMark(Mark::Cursor(0))),
                object: Some(TextObject {
                    kind: Kind::Char,
                    offset: Offset::Forward(1, Mark::Cursor(0)),
                }),
            },
        );
        keymap.bind_key(
            Key::Ctrl('h'),
            Command {
                number: 1,
                action: Action::Operation(Operation::DeleteFromMark(Mark::Cursor(0))),
                object: Some(TextObject {
                    kind: Kind::Char,
                    offset: Offset::Backward(1, Mark::Cursor(0)),
                }),
            },
        );

        keymap.bind_key(Key::Ctrl('d'), Command::duplicate_selection());
        keymap.bind_key(Key::Ctrl('k'), Command::delete_selection());
        keymap.bind_key(Key::Ctrl('x'), Command::cut_selection());
        keymap.bind_key(Key::Ctrl('c'), Command::copy_selection());
        keymap.bind_key(Key::Ctrl('v'), Command::paste());
        keymap.bind_key(Key::CtrlUp, Command::move_selection(false));
        keymap.bind_key(Key::CtrlDown, Command::move_selection(true));

        // History
        keymap.bind_key(Key::Ctrl('z'), Command::undo());
        keymap.bind_key(Key::Ctrl('y'), Command::redo());


    }
}
*/
