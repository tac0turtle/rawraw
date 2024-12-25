// use std::borrow::Cow;
//
// #[derive(Debug, PartialEq)]
// pub enum UnescapeError {
//     InvalidEscape(usize),
//     InvalidUnicode(usize),
//     UnterminatedString,
//     InvalidQuotes,
// }
//
// pub fn unescape_json_string(s: &str) -> Result<Cow<str>, UnescapeError> {
//     // If the string starts with a quote, it must end with one
//     let (s, is_quoted) = if s.starts_with('"') {
//         if !s.ends_with('"') {
//             return Err(UnescapeError::UnterminatedString);
//         }
//         (&s[1..s.len()-1], true)
//     } else {
//         (s, false)
//     };
//
//     // Quick check: if there are no backslashes, we can return the string as-is
//     if !s.contains('\\') {
//         return Ok(Cow::Borrowed(s));
//     }
//
//     let mut chars = s.char_indices();
//     let mut res = String::with_capacity(s.len());
//     let mut pos = 0;
//
//     while let Some((i, c)) = chars.next() {
//         if c != '\\' {
//             res.push(c);
//             continue;
//         }
//
//         // Copy any skipped chars
//         if i > pos {
//             res.push_str(&s[pos..i]);
//         }
//
//         // Handle escape sequence
//         let escaped = chars.next()
//             .ok_or(UnescapeError::InvalidEscape(i))?;
//
//         match escaped.1 {
//             '"' => res.push('"'),
//             '\\' => res.push('\\'),
//             '/' => res.push('/'),
//             'b' => res.push('\u{0008}'),
//             'f' => res.push('\u{000C}'),
//             'n' => res.push('\n'),
//             'r' => res.push('\r'),
//             't' => res.push('\t'),
//             'u' => {
//                 // Read exactly 4 hex digits
//                 let mut code = 0u32;
//                 for _ in 0..4 {
//                     let (_, hex) = chars.next()
//                         .ok_or(UnescapeError::InvalidUnicode(i))?;
//
//                     let digit = hex.to_digit(16)
//                         .ok_or(UnescapeError::InvalidUnicode(i))?;
//                     code = (code << 4) | digit;
//                 }
//
//                 if (0xD800..=0xDBFF).contains(&code) {
//                     // High surrogate, must be followed by low surrogate
//                     match chars.next() {
//                         Some((_, '\\')) => (),
//                         _ => return Err(UnescapeError::InvalidUnicode(i)),
//                     }
//                     match chars.next() {
//                         Some((_, 'u')) => (),
//                         _ => return Err(UnescapeError::InvalidUnicode(i)),
//                     }
//
//                     let mut low_code = 0u32;
//                     for _ in 0..4 {
//                         let (_, hex) = chars.next()
//                             .ok_or(UnescapeError::InvalidUnicode(i))?;
//
//                         let digit = hex.to_digit(16)
//                             .ok_or(UnescapeError::InvalidUnicode(i))?;
//                         low_code = (low_code << 4) | digit;
//                     }
//
//                     if !(0xDC00..=0xDFFF).contains(&low_code) {
//                         return Err(UnescapeError::InvalidUnicode(i));
//                     }
//
//                     let unicode = (((code - 0xD800) << 10) | (low_code - 0xDC00)) + 0x10000;
//                     res.push(char::from_u32(unicode)
//                         .ok_or(UnescapeError::InvalidUnicode(i))?);
//                 } else if (0xDC00..=0xDFFF).contains(&code) {
//                     // Unexpected low surrogate
//                     return Err(UnescapeError::InvalidUnicode(i));
//                 } else {
//                     res.push(char::from_u32(code)
//                         .ok_or(UnescapeError::InvalidUnicode(i))?);
//                 }
//             }
//             _ => return Err(UnescapeError::InvalidEscape(i)),
//         }
//
//         pos = escaped.0 + 1;
//     }
//
//     // Copy any remaining chars
//     if pos < s.len() {
//         res.push_str(&s[pos..]);
//     }
//
//     Ok(Cow::Owned(res))
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_no_escapes() {
//         assert_eq!(unescape_json_string("\"simple string\"").unwrap(), "simple string");
//         assert_eq!(unescape_json_string("simple string").unwrap(), "simple string");
//         match unescape_json_string("simple string").unwrap() {
//             Cow::Borrowed(_) => (),
//             Cow::Owned(_) => panic!("Expected borrowed string"),
//         }
//     }
//
//     #[test]
//     fn test_simple_escapes() {
//         assert_eq!(
//             unescape_json_string(r#""hello\"world""#).unwrap(),
//             r#"hello"world"#
//         );
//         assert_eq!(
//             unescape_json_string(r#""hello\\world""#).unwrap(),
//             r#"hello\world"#
//         );
//         assert_eq!(
//             unescape_json_string(r#""hello\nworld""#).unwrap(),
//             "hello\nworld"
//         );
//     }
//
//     #[test]
//     fn test_unicode_escapes() {
//         assert_eq!(
//             unescape_json_string(r#""\u0041\u0042C""#).unwrap(),
//             "ABC"
//         );
//         // Surrogate pair for 🦀 (crab emoji)
//         assert_eq!(
//             unescape_json_string(r#""\ud83e\udd80""#).unwrap(),
//             "🦀"
//         );
//     }
//
//     #[test]
//     fn test_invalid_escapes() {
//         assert!(matches!(
//             unescape_json_string(r#""\x""#),
//             Err(UnescapeError::InvalidEscape(_))
//         ));
//     }
//
//     #[test]
//     fn test_invalid_unicode() {
//         // Invalid hex digits
//         assert!(matches!(
//             unescape_json_string(r#""\u12zz""#),
//             Err(UnescapeError::InvalidUnicode(_))
//         ));
//         // Incomplete unicode
//         assert!(matches!(
//             unescape_json_string(r#""\u12""#),
//             Err(UnescapeError::InvalidUnicode(_))
//         ));
//         // Invalid surrogate pair
//         assert!(matches!(
//             unescape_json_string(r#""\ud800\u0041""#),
//             Err(UnescapeError::InvalidUnicode(_))
//         ));
//     }
//
//     #[test]
//     fn test_unterminated() {
//         assert!(matches!(
//             unescape_json_string("\"unterminated"),
//             Err(UnescapeError::UnterminatedString)
//         ));
//     }
//
//     #[test]
//     fn test_performance() {
//         let long_string = "a".repeat(1000);
//         let quoted = format!("\"{}\"", long_string);
//         match unescape_json_string(&quoted).unwrap() {
//             Cow::Borrowed(_) => (),
//             Cow::Owned(_) => panic!("Expected borrowed string for unescaped input"),
//         }
//     }
//
//     #[test]
//     fn test_unquoted_escapes() {
//         assert_eq!(unescape_json_string(r"hello\"world").unwrap(), r#"hello"world"#);
//         assert_eq!(unescape_json_string(r"hello\\world").unwrap(), r#"hello\world"#);
//         assert_eq!(unescape_json_string(r"hello\nworld").unwrap(), "hello\nworld");
//     }
// }