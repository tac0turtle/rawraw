//! GENERATED CODE -- DO NOT EDIT!

#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive
)]
#[repr(u16)]
pub enum SyntaxKind {
    ROOT,
    ERROR,
    #[num_enum(catch_all)]
    UNKNOWN(u16),
    WHITESPACE,
    COMMENT,
    INTERFACE,
    IDENT,
    L_BRACKET,
    R_BRACKET,
    HANDLER,
    FN,
    L_PAREN,
    R_PAREN,
    COLON,
    L_BRACE,
    R_BRACE,
}
