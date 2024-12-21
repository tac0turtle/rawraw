use crate::db::FileSource;
use crate::frontend::ast::File;
use crate::frontend::parser;
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
                if let Some(name) = it.name() {
                    scope.type_ids.insert(
                        name.text().to_string(),
                        TypeID::new(db, name.text().to_string()),
                    );
                }
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
    Fn()
}