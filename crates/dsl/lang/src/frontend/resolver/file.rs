use crate::frontend::ast::{File, Item};
use crate::frontend::resolver::scope::{ScopeBuilder, ScopeProvider};
use crate::frontend::resolver::symbol::PrimitiveSymbol;

impl ScopeProvider for File {
    fn provide_scope(&self, scope: &mut ScopeBuilder) {
        scope.define_primitive("u128", PrimitiveSymbol::U128);
        scope.define_primitive("Str", PrimitiveSymbol::Str);
        scope.define_primitive("AccountID", PrimitiveSymbol::AccountID);
        
        for item in self.items() {
            match item {
                Item::Interface(it) => scope.put_into_scope(it),
                Item::Object(it) => scope.put_into_scope(it),
                _ => {}
            }
        }
    }
}