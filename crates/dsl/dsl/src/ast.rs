use std::ops::Range;

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Interface(ItemInterface),
    Handler(Handler),
    Impl(Impl),
}

#[derive(Debug, Clone)]
pub struct ItemInterface {
    pub name: Ident,
    pub items: Vec<InterfaceItem>,
}

#[derive(Debug, Clone)]
pub struct Ident {
    value: String,
    // span: Span,
}

#[derive(Debug, Clone)]
pub struct Span(pub Range<usize>);

#[derive(Debug, Clone)]
pub enum InterfaceItem {
    Fn(FnSignature),
    Event(Struct),
    Struct(Struct),
    // Enum(Enum),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: Ident,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Ident(Ident),
    Array(Box<Type>),
}

#[derive(Debug, Clone)]
pub struct FnSignature {
    pub name: Ident,
    pub fn_type: FnType,
    pub args: Vec<FnArg>,
    pub events: Vec<Ident>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct FnArg {
    pub name: Ident,
    pub ty: Type,
    pub key: bool,
}

#[derive(Debug, Clone)]
pub enum FnType {
    Tx,
    Query,
    Pure,
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone)]
pub struct Handler {
    pub name: Ident,
    pub items: Vec<HandlerItem>,
}

#[derive(Debug, Clone)]
pub struct Impl {}

#[derive(Debug, Clone)]
pub enum HandlerItem {
    Map(Map),
    Client(),
}

#[derive(Debug, Clone)]
pub struct Map {
    pub name: Ident,
    pub key_fields: Vec<MapField>,
    pub value_fields: Vec<MapField>,
}

#[derive(Debug, Clone)]
pub struct MapField {
    pub name: Ident,
    pub ty: Type,
}

pub struct Block {
    stmts: Vec<Stmt>,
}

pub enum Stmt {
    Local(Local),
    Expr(Expr),
}

pub struct Local {
    name: String,
    ty: String,
}

pub enum Expr {
    Call(ExprCall),
}

pub struct ExprCall {
    pub name: String,
}

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Ident {
            value: value.to_string(),
        }
    }
}
