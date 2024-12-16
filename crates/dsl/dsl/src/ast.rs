use std::ops::Range;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Interface(ItemInterface),
    Handler(Handler),
    Impl(Impl),
}

#[derive(Debug)]
pub struct ItemInterface {
    pub name: String,
    pub items: Vec<InterfaceItem>,
}

#[derive(Debug)]
pub struct Ident {
    pub value: String,
    pub span: Span,
}

#[derive(Debug)]
pub struct Span(pub Range<usize>);

#[derive(Debug)]
enum InterfaceItem {
    Fn(InterfaceItemFn),
    // Event(Struct),
    // Struct(Struct),
    // Enum(Enum),
}

#[derive(Debug)]
pub struct InterfaceItemFn {
    pub name: String,
    pub fn_type: FnType,
    pub visibility: Visibility,
    pub fn_args: Vec<FnArg>,
}

#[derive(Debug)]
pub struct FnArg {
    pub name: String,
    pub ty: String,
    pub key: bool,
}

#[derive(Debug)]
pub enum FnType {
    Tx,
    Query,
    Pure,
}

#[derive(Debug)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug)]
pub struct Handler {}

#[derive(Debug)]
pub struct Impl {}

#[derive(Debug)]
pub enum HandlerItem {
    Collection(),
    Client(),
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
