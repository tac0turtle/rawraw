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
    IDENT,
    INTERFACE_KW,
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
    MUT_KW,
    REF_KW,
    TRANSFER_KW,
    EMITS_KW,
    L_SQUARE,
    R_SQUARE,
    STRUCT_KW,
    EVENT_KW,
    ACCOUNT_SCOPED_KW,
    MAP_KW,
    VAR_KW,
    R_ARROW,
    CLIENT_KW,
    IMPL_KW,
    FOR_KW,
    DOT,
    EQ,
    IN_KW,
    TEST_KW,
    NAME,
    NAME_REF,
    FILE,
    ITEM,
    INTERFACE,
    OBJECT,
    IMPL,
    TEST,
    INTERFACE_ITEM,
    INTERFACE_FN,
    STRUCT,
    EVENT,
    MAP_COLLECTION,
    VAR_COLLECTION,
    FN_SIGNATURE,
    OBJECT_ITEM,
    CLIENT,
    IMPL_FN,
    FN_TYPE,
    FN_PARAM_LIST,
    FN_EVENTS,
    FN_RET,
    FN_PARAM,
    FN_PARAM_MODIFIER,
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
    FN_BLOCK,
    STMT,
    STMT_EXPR,
    EXPR,
    EXPR_PAREN,
    NAME_EXPR,
    EXPR_CALL,
    FIELD_REF_EXPR,
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
            SyntaxKind::MUT_KW => true,
            SyntaxKind::REF_KW => true,
            SyntaxKind::TRANSFER_KW => true,
            SyntaxKind::EMITS_KW => true,
            SyntaxKind::STRUCT_KW => true,
            SyntaxKind::EVENT_KW => true,
            SyntaxKind::ACCOUNT_SCOPED_KW => true,
            SyntaxKind::MAP_KW => true,
            SyntaxKind::VAR_KW => true,
            SyntaxKind::CLIENT_KW => true,
            SyntaxKind::IMPL_KW => true,
            SyntaxKind::FOR_KW => true,
            SyntaxKind::IN_KW => true,
            SyntaxKind::TEST_KW => true,
            _ => false,
        }
    }
}
