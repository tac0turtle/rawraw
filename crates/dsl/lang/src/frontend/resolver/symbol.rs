use rowan::ast::AstNode;
use crate::frontend::ast::*;
use crate::frontend::resolver::node_id::NodeId;
use crate::frontend::syntax::IXCLanguage;

pub trait SymbolDefiner: AstNode<Language = IXCLanguage> {
    fn get_name(&self) -> Option<Name>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SymbolId {
    Node(NodeId),
    Primitive(PrimitiveSymbol)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveSymbol {
    Str,
    U128,
    AccountID,
}

impl SymbolDefiner for Interface {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}

impl SymbolDefiner for Object {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}

impl SymbolDefiner for Event {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}

impl SymbolDefiner for InterfaceFn {
    fn get_name(&self) -> Option<Name> {
        self.sig().map(|it| it.name()).flatten()
    }
}

impl SymbolDefiner for MapCollection {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}

impl SymbolDefiner for VarCollection {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}