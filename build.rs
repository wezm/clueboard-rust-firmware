use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;
use std::path::PathBuf;

const SYMBOL_MAP: [(char, (bool, &str)); 33] = [
    ('`', (false, "Grave")),
    ('-', (false, "Minus")),
    ('=', (false, "Equal")),
    ('[', (false, "LBracket")),
    (']', (false, "RBracket")),
    ('\\', (false, "Bslash")),
    (';', (false, "SColon")),
    ('\'', (false, "Quote")),
    (',', (false, "Comma")),
    ('.', (false, "Dot")),
    ('/', (false, "Slash")),
    (' ', (false, "Space")),
    // Shift keys
    ('~', (true, "Grave")),
    ('!', (true, "Kb1")),
    ('@', (true, "Kb2")),
    ('#', (true, "Kb3")),
    ('$', (true, "Kb4")),
    ('%', (true, "Kb5")),
    ('^', (true, "Kb6")),
    ('&', (true, "Kb7")),
    ('*', (true, "Kb8")),
    ('(', (true, "Kb9")),
    (')', (true, "Kb0")),
    ('_', (true, "Minus")),
    ('+', (true, "Equal")),
    ('{', (true, "LBracket")),
    ('}', (true, "RBracket")),
    ('|', (true, "Bslash")),
    (':', (true, "SColon")),
    ('"', (true, "Quote")),
    ('<', (true, "Comma")),
    ('>', (true, "Dot")),
    ('?', (true, "Slash")),
];

fn main() {
    let macros_src = include_str!("src/macros.txt");
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("macros.rs");
    let mut out_file = File::create(&output_path).expect("unable to create output macro file");
    let shift_map: HashMap<char, (bool, &str)> = HashMap::from_iter(SYMBOL_MAP);

    for line in macros_src.lines() {
        if line.starts_with('#') {
            continue;
        }

        let (const_name, rest) = line.split_once(':').unwrap();
        let keys = if rest.starts_with(' ') {
            // Skip space in between : and definition
            &rest[1..]
        } else {
            rest
        };

        write!(
            out_file,
            "const {}: Action = Action::Sequence {{ events: &[",
            const_name
        )
        .unwrap();
        for ch in keys.chars() {
            if !ch.is_ascii() {
                panic!("line contains non-ASCII keys: {}", line)
            }

            match ch {
                '0'..='9' => {
                    write!(out_file, "SequenceEvent::Press(Kb{key}), SequenceEvent::Release(Kb{key}), ", key=ch).unwrap();
                }
                'a'..='z' => {
                    let key = ch.to_ascii_uppercase();
                    out_file.write_all(press_release(key).as_bytes()).unwrap();
                }
                'A'..='Z' => {
                    write!(out_file, "SequenceEvent::Press(LShift), ").unwrap();
                    out_file.write_all(press_release(ch).as_bytes()).unwrap();
                    write!(out_file, "SequenceEvent::Release(LShift), ").unwrap();
                }
                _ => {
                    if let Some(&(shift, keys)) = shift_map.get(&ch) {
                        if shift {
                            write!(out_file, "SequenceEvent::Press(LShift), SequenceEvent::Press({key}), SequenceEvent::Release({key}), SequenceEvent::Release(LShift), ", key=keys).unwrap();
                        } else {
                            write!(
                                out_file,
                                "SequenceEvent::Press({key}), SequenceEvent::Release({key}), ",
                                key = keys
                            )
                            .unwrap();
                        }
                    } else {
                        panic!("unhandled char: {}", ch);
                    }
                }
            }
        }
        writeln!(out_file, "] }};").unwrap();
    }
}

fn press_release(key: char) -> String {
    format!(
        "SequenceEvent::Press({key}), SequenceEvent::Release({key}), ",
        key = key
    )
}
