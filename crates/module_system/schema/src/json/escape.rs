extern crate std;

use std::io::{Write};

/// Escapes a JSON string by performing only the required escaping and writes the result to the provided writer.
pub(crate) fn escape_json<W: Write>(input: &str, writer: &mut W) -> std::io::Result<()> {
    /// Converts a 4-bit nibble to its corresponding uppercase hexadecimal ASCII byte.
    #[inline]
    fn to_hex(nibble: u8) -> u8 {
        match nibble {
            0..=9 => b'0' + nibble,
            10..=15 => b'A' + (nibble - 10),
            _ => b'0', // Fallback, though nibble should never exceed 15
        }
    }

    writer.write_all(b"\"")?;

    for c in input.chars() {
        match c {
            // Required Escapes
            '"' => {
                writer.write_all(b"\\\"")?;
            },
            '\\' => {
                writer.write_all(b"\\\\")?;
            },
            '\u{0008}' => { // Backspace
                writer.write_all(b"\\b")?;
            },
            '\u{000C}' => { // Form feed
                writer.write_all(b"\\f")?;
            },
            '\n' => { // Newline
                writer.write_all(b"\\n")?;
            },
            '\r' => { // Carriage return
                writer.write_all(b"\\r")?;
            },
            '\t' => { // Tab
                writer.write_all(b"\\t")?;
            },
            // Control Characters (U+0000 to U+001F) not covered above
            c if c <= '\u{001F}' => {
                let code = c as u32;

                // Start Unicode escape sequence
                writer.write_all(b"\\u00")?;

                // Extract the high and low 4 bits of the character code
                let high_nibble = ((code & 0xF0) >> 4) as u8;
                let low_nibble = (code & 0x0F) as u8;

                // Convert each nibble to its hexadecimal representation
                writer.write_all(&[to_hex(high_nibble), to_hex(low_nibble)])?;
            },
            // All other characters are written as-is (UTF-8 encoded)
            _ => {
                // Since `c` is a `char`, it may consist of multiple bytes in UTF-8.
                // We convert it to its UTF-8 byte representation and write directly.
                writer.write_all(c.encode_utf8(&mut [0u8; 4]).as_bytes())?;
            },
        }
    }

    writer.write_all(b"\"")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use super::*;

    #[test]
    fn test_escape_json_str_to_writer() {
        let test_cases = vec![
            (
                "Simple string",
                "Hello, World!",
                "Hello, World!",
            ),
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
            (
                "Non-escaped Unicode",
                "Emoji: 😃",
                "Emoji: 😃",
            ),
        ];

        for (description, input, expected) in test_cases {
            let mut buffer = Vec::new();
            escape_json(input, &mut buffer).expect("Failed to escape string");
            let escaped_str = String::from_utf8(buffer).expect("Invalid UTF-8");
            assert_eq!(escaped_str, expected, "Failed on: {}", description);
        }
    }
}
