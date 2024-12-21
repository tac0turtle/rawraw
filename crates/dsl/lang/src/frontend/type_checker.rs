use crate::db::FileSource;
use crate::frontend::ast::{Expr, File};
use crate::frontend::parser;
use crate::frontend::syntax::{SyntaxElement, SyntaxKind, SyntaxNode};
use rowan::ast::AstNode;
use salsa::Database;
use std::collections::HashMap;

#[salsa::tracked]
pub struct TypedAST<'db> {}

#[salsa::tracked]
pub fn type_check(db: &dyn Database, ast: parser::ParsedAST<'_>) -> TypedAST<'_> {
    let root = ast.syntax(&db);
    if let Some(file) = File::cast(root) {
        check_file(db, file);
    }
    TypedAST::new(db)
}

#[salsa::interned]
pub struct TypeID<'db> {
    #[return_ref]
    pub name: String,
}

pub enum Type<'db> {
    Interface(InterfaceTy<'db>),
    Struct(StructTy<'db>),
}

pub struct TypeId<'db> {
    #[return_ref]
    pub name: String,
}

pub struct FnTy<'db> {
    #[return_ref]
    pub name: String,
    #[return_ref]
    pub params: Vec<TypeId<'db>>,
    pub ret: TypeId<'db>,
}

#[salsa::tracked]
pub struct InterfaceTy<'db> {
    #[id]
    pub id: TypeID<'db>,
    #[return_ref]
    pub name: String,
}

pub enum InterfaceTyItem<'db> {
    Struct(StructTy<'db>),
}

#[salsa::tracked]
pub struct StructTy<'db> {
    #[id]
    pub id: TypeID<'db>,
    pub fields: Vec<StructTyField<'db>>,
}

#[salsa::tracked]
pub struct StructTyField<'db> {
    pub name: String,
    pub ty: TypeID<'db>,
}

fn check_file(db: &dyn Database, file: File) {
    let mut scope = Scope::default();
    for item in file.items() {
        match item {
            File::Item::Interface(it) => {
                // if let Some(name) = it.name() {
                //     scope.type_ids.insert(
                //         name.text().to_string(),
                //         TypeID::new(db, name.text().to_string()),
                //     );
                // }
            }
            // File::Item::Object(it) => check_object(db, it, &types),
            // File::Item::Impl(it) => check_impl(db, it, &types),
            // File::Item::Test(it) => check_test(db, it, &types),
            _ => {}
        }
    }
}

pub struct ValueID<'db> {
    pub name: String,
}

#[salsa::interned]
pub struct SymbolID<'db> {
    #[return_ref]
    pub name: String,
}

#[derive(Default)]
pub struct Scope<'db> {
    parent: Option<Scope<'db>>,
    symbols: HashMap<SymbolID<'db>, Item<'db>>,
}

pub enum Item<'db> {
    Type(Type<'db>),
    Value(Value<'db>),
}

pub enum Value<'db> {
    Fn(),
}

struct Checker<'db> {
    db: &'db dyn Database,
}

impl<'db> Checker<'db> {
    fn check_expr(&self, expr: Expr, scope: &Scope<'db>) -> Option<&'db Type<'db>> {
        match expr {
            Expr::NameExpr(name) => {
                // get name, or error
                // resolve name to item
                // get item type
            }
            Expr::ExprCall(call) => {
                let callee = call.expr()?;
                let callee_type = self.check_expr(callee, scope)?;
                // check if callee is a function
                let args = call.args()?;
                // check number of args
                for arg in args.args() {
                    let expr = arg.expr()?;
                    self.check_expr(expr, scope)?;
                    // check arg is assignable to function param
                }
                // if let Some(callee) = call.expr() {
                //     if let callee_type = self.check_expr(callee, scope) {
                //         // check args
                //         if let Some(args) = call.args() {
                //             for arg in args.args() {
                //                 if let Some(expr) = arg.expr() {
                //                     self.check_expr(expr, scope);
                //                 } else {
                //                     // error
                //                 }
                //             }
                //         } else {
                //             // error
                //         }
                //     } else {
                //         // error
                //     }
                // } else {
                //     // error
                // }
            }
            Expr::ExprParen(paren) => {}
            _ => {}
        }
        todo!()
    }
}

impl<'db> Type<'db> {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        todo!()
    }
}

// #[salsa::interned]
// pub struct SymbolPath<'db> {
//     #[return_ref]
//     pub file: String,
//     #[return_ref]
//     pub item_path:ItemPath<'db>,
// }
//
// pub type ChildPath<'db, ItemT> = Option<(String, ItemT)>;
//
// pub struct ItemPath<'db, T> {
//     pub name: String,
//     pub children: Vec<ChildPath<'db, T>>,
// }
//
// pub struct InterfacePath<'db> {
//     #[return_ref]
//     pub name: String,
//     pub items: InterfaceItemPath<'db>,
// }
//
// pub struct InterfaceItemPath<'db> {
// }

#[salsa::interned]
pub struct SymbolPath<'db> {
    #[return_ref]
    pub path: SyntaxTreePath,
}

pub struct SyntaxTreePath(pub Vec<PathSegment>);

impl SyntaxTreePath {
    pub fn resolve(&self, node: &SyntaxNode) -> Option<SyntaxElement> {
        let mut maybe_node = Some(node.clone());
        let mut elem = None;
        for segment in self.0.iter() {
            let node = maybe_node?;
            let elem = segment.resolve(&node)?;
            maybe_node = elem.into_node()
        }
        elem
    }
}

pub struct PathSegment {
    kind: SyntaxKind,
    index: usize,
}

impl PathSegment {
    fn resolve(&self, node: &SyntaxNode) -> Option<SyntaxElement> {
        let mut idx = 0;
        node.children_with_tokens().find_map(|elem| {
            if elem.kind() == self.kind {
                if idx == self.index {
                    return Some(elem);
                }
                idx += 1;
            }
            None
        })
    }
}

