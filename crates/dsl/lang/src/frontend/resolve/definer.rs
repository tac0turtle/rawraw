use rowan::ast::AstNode;
use crate::frontend::ast::*;

pub trait ItemDefiner: AstNode {
    fn get_name(&self) -> Option<Name>;
}

impl ItemDefiner for Interface {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}

impl ItemDefiner for Object {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}

impl ItemDefiner for Struct {
    fn get_name(&self) -> Option<Name> {
        None
    }
}

impl ItemDefiner for Event {
    fn get_name(&self) -> Option<Name> {
        self.name()
    }
}