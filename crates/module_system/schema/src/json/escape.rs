extern crate std;
use std::io::Write;
use std::io;

pub fn write_json_string<W: Write>(writer: &mut W, s: &str) -> io::Result<()> {
    writer.write_all(b"\"")?;

    for c in s.chars() {
        match c {
            '"' => writer.write_all(b"\\\"")?,
            '\\' => writer.write_all(b"\\\\")?,
            '\x08' => writer.write_all(b"\\b")?,  // backspace
            '\x0C' => writer.write_all(b"\\f")?,  // form feed
            '\n' => writer.write_all(b"\\n")?,
            '\r' => writer.write_all(b"\\r")?,
            '\t' => writer.write_all(b"\\t")?,
            c if c.is_control() => {
                write!(writer, "\\u{:04x}", c as u32)?;
            }
            c if c.is_ascii() => {
                writer.write_all(&[c as u8])?;
            }
            c => {
                // For non-ASCII chars, we need to handle surrogate pairs for UTF-16
                let c = c as u32;
                if c < 0x10000 {
                    write!(writer, "\\u{:04x}", c)?;
                } else {
                    // Split into high and low surrogate pair
                    let c = c - 0x10000;
                    let high = 0xD800 + (c >> 10);
                    let low = 0xDC00 + (c & 0x3FF);
                    write!(writer, "\\u{:04x}\\u{:04x}", high, low)?;
                }
            }
        }
    }

    writer.write_all(b"\"")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate std;
    use alloc::string::String;
    use alloc::vec::Vec;
    use std::io::BufWriter;
    use super::*;

    fn escape_to_string(s: &str) -> String {
        let mut output = Vec::new();
        write_json_string(&mut output, s).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_basic_string() {
        assert_eq!(escape_to_string("hello"), "\"hello\"");
    }

    #[test]
    fn test_escape_quotes() {
        assert_eq!(escape_to_string("hello \"world\""), "\"hello \\\"world\\\"\"");
    }

    #[test]
    fn test_escape_backslash() {
        assert_eq!(escape_to_string("C:\\path\\to\\file"), "\"C:\\\\path\\\\to\\\\file\"");
    }

    #[test]
    fn test_escape_control_chars() {
        assert_eq!(escape_to_string("hello\nworld"), "\"hello\\nworld\"");
        assert_eq!(escape_to_string("hello\tworld"), "\"hello\\tworld\"");
        assert_eq!(escape_to_string("hello\rworld"), "\"hello\\rworld\"");
        assert_eq!(escape_to_string("hello\x08world"), "\"hello\\bworld\"");
        assert_eq!(escape_to_string("hello\x0Cworld"), "\"hello\\fworld\"");
    }

    #[test]
    fn test_escape_unicode() {
        assert_eq!(escape_to_string("hello\u{0001}world"), "\"hello\\u0001world\"");
        assert_eq!(escape_to_string("hello🦀world"), "\"hello\\ud83e\\udd80world\"");
    }

    #[test]
    fn test_streaming_write() {
        let mut output = Vec::new();
        {
            let mut writer = BufWriter::new(&mut output);
            write_json_string(&mut writer, "hello\nworld").unwrap();
            writer.flush().unwrap();
        }
        assert_eq!(String::from_utf8(output).unwrap(), "\"hello\\nworld\"");    }
}