extern crate std;

use core::fmt::Write;

/// Escapes a JSON string by performing only the required escaping and writes the result to the provided writer.
pub(crate) fn escape_json<W: Write>(input: &str, writer: &mut W) -> core::fmt::Result {
    writer.write_char('"')?;
    escape_json_inner(input, writer)?;
    writer.write_char('"')?;
    Ok(())
}

/// JSON string escaping without the surrounding quotes.
fn escape_json_inner<W: Write>(input: &str, writer: &mut W) -> core::fmt::Result {
    /// Converts a 4-bit nibble to its corresponding uppercase hexadecimal ASCII byte.
    #[inline]
    fn to_hex(nibble: u8) -> char {
        let nibble = match nibble {
            0..=9 => b'0' + nibble,
            10..=15 => b'A' + (nibble - 10),
            _ => b'0', // Fallback, though nibble should never exceed 15
        };
        char::from(nibble)
    }

    for c in input.chars() {
        match c {
            // Required Escapes
            '"' => {
                writer.write_str("\\\"")?;
            }
            '\\' => {
                writer.write_str("\\\\")?;
            }
            '\u{0008}' => {
                // Backspace
                writer.write_str("\\b")?;
            }
            '\u{000C}' => {
                // Form feed
                writer.write_str("\\f")?;
            }
            '\n' => {
                // Newline
                writer.write_str("\\n")?;
            }
            '\r' => {
                // Carriage return
                writer.write_str("\\r")?;
            }
            '\t' => {
                // Tab
                writer.write_str("\\t")?;
            }
            // Control Characters (U+0000 to U+001F) not covered above
            c if c <= '\u{001F}' => {
                let code = c as u32;

                // Start Unicode escape sequence
                writer.write_str("\\u00")?;

                // Extract the high and low 4 bits of the character code
                let high_nibble = ((code & 0xF0) >> 4) as u8;
                let low_nibble = (code & 0x0F) as u8;

                // Convert each nibble to its hexadecimal representation
                writer.write_char(to_hex(high_nibble))?;
                writer.write_char(to_hex(low_nibble))?;
            }
            // All other characters are written as-is (UTF-8 encoded)
            _ => {
                // Since `c` is a `char`, it may consist of multiple bytes in UTF-8.
                // We convert it to its UTF-8 byte representation and write directly.
                writer.write_char(c)?
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::encoder::Writer;
    use allocator_api2::vec;
    use allocator_api2::vec::Vec;

    #[test]
    fn test_escape_json_str_to_writer() {
        let test_cases = vec![
            ("Simple string", "Hello, World!", "Hello, World!"),
            (
                "String with quotes",
                "She said, \"Hello!\"",
                "She said, \\\"Hello!\\\"",
            ),
            (
                "String with backslashes",
                "C:\\Program Files\\App",
                "C:\\\\Program Files\\\\App",
            ),
            (
                "String with control characters",
                "Line1\nLine2\r\nTab\tEnd",
                "Line1\\nLine2\\r\\nTab\\tEnd",
            ),
            (
                "String with Unicode control character",
                "Null char:\u{0000}",
                "Null char:\\u0000",
            ),
            (
                "Mixed string",
                "Quote: \", Backslash: \\, Tab:\t, Unicode:\u{001F}",
                "Quote: \\\", Backslash: \\\\, Tab:\\t, Unicode:\\u001F",
            ),
            ("Non-escaped Unicode", "Emoji: 😃", "Emoji: 😃"),
        ];

        for (description, input, expected) in test_cases {
            let mut out = Vec::new();
            let mut buffer = Writer(&mut out);
            escape_json_inner(input, &mut buffer).expect("Failed to escape string");
            let escaped_str = core::str::from_utf8(out.as_slice()).expect("Invalid UTF-8");
            assert_eq!(escaped_str, expected, "Failed on: {}", description);
        }
    }
}
