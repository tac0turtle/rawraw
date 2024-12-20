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
    EOF,
    ERROR,
    ERROR_NODE,
    #[num_enum(catch_all)]
    UNKNOWN(u16),
    WHITESPACE,
    LINE_COMMENT,
    INTERFACE_KW,
    IDENT,
    L_CURLY,
    R_CURLY,
    SEMICOLON,
    OBJECT_KW,
    L_PAREN,
    R_PAREN,
    TX_KW,
    QUERY_KW,
    PURE_KW,
    KEY_KW,
    COLON,
    COMMA,
    EMITS_KW,
    L_SQUARE,
    R_SQUARE,
    STRUCT_KW,
    EVENT_KW,
    SCOPED_KW,
    MAP_KW,
    R_ARROW,
    CLIENT_KW,
    IMPL_KW,
    FOR_KW,
    DOT,
    EQ,
    IN_KW,
    FILE,
    ITEM,
    INTERFACE,
    OBJECT,
    IMPL,
    INTERFACE_ITEM,
    INTERFACE_FN,
    STRUCT,
    EVENT,
    FN_SIGNATURE,
    OBJECT_ITEM,
    MAP_COLLECTION,
    CLIENT,
    FN_TYPE,
    FN_PARAM_LIST,
    FN_EVENTS,
    FN_RET,
    FN_PARAM,
    TYPE,
    TYPE_IDENT,
    TYPE_ARRAY,
    STRUCT_FIELD,
    MAP_KEY_FIELDS,
    MAP_VALUE_FIELDS,
    MAP_FIELD,
    CLIENT_TYPES,
    CLIENT_TYPE,
    IMPL_FOR,
    IMPL_ITEM,
    IMPL_FN,
    FN_BLOCK,
    STMT,
    STMT_EXPR,
    EXPR,
    EXPR_PAREN,
    NAME_EXPR,
    EXPR_CALL,
    FIELD_EXPR,
    ARG_LIST,
    EXPR_BINARY,
    BINARY_OP,
    RHS,
    ARG,
    EXPR_CONSTRUCT,
    EXPR_CONSTRUCT_FIELD_LIST,
    EXPR_CONSTRUCT_FIELD,
    FOR_STMT,
}
impl SyntaxKind {
    pub fn is_keyword(&self) -> bool {
        match self {
            SyntaxKind::INTERFACE_KW => true,
            SyntaxKind::OBJECT_KW => true,
            SyntaxKind::TX_KW => true,
            SyntaxKind::QUERY_KW => true,
            SyntaxKind::PURE_KW => true,
            SyntaxKind::KEY_KW => true,
            SyntaxKind::EMITS_KW => true,
            SyntaxKind::STRUCT_KW => true,
            SyntaxKind::EVENT_KW => true,
            SyntaxKind::SCOPED_KW => true,
            SyntaxKind::MAP_KW => true,
            SyntaxKind::CLIENT_KW => true,
            SyntaxKind::IMPL_KW => true,
            SyntaxKind::FOR_KW => true,
            SyntaxKind::IN_KW => true,
            _ => false,
        }
    }
}
