use rowan::ast::{AstNode, AstPtr};
use crate::frontend::ast::{File, FnParam, ImplFn, Interface, Item, Object, ObjectItem};
use crate::frontend::type_checker::item::ItemRef;
use crate::frontend::type_checker::scope::ScopeBuilder;

pub trait BindSymbols: AstNode {
    fn bind_symbols<'db>(&self, builder: &mut ScopeBuilder<'db>);
    // fn item_ref<'db>(ptr: AstPtr<Self>) -> ItemRef<'db>;
}

// impl BindSymbols for File {
//     fn bind_symbols<'db>(&self, builder: &mut ScopeBuilder<'db>) {
//     }
//
//     fn item_ref(ptr: AstPtr<Self>) -> ItemRef<'static> {
//         ItemRef::File(ptr)
//     }
// }
//
// impl BindSymbols for Object {
//     fn bind_symbols<'db>(&self, builder: &mut ScopeBuilder<'db>) {
//     }
//
//     fn item_ref(ptr: AstPtr<Self>) -> ItemRef<'static> {
//         ItemRef::Object(ptr)
//     }
// }
//
// impl BindSymbols for ImplFn {
//     fn bind_symbols<'db>(&self, builder: &mut ScopeBuilder<'db>) {
//         // Implementation unchanged
//     }
//
//     fn item_ref(ptr: AstPtr<Self>) -> ItemRef<'static> {
//         ItemRef::ImplFn(ptr)
//     }
// }
//
// // impl BindSymbols for Interface {
// //     fn item_ref(ptr: AstPtr<Self>) -> ItemRef<'static> {
// //         ItemRef::Interface(ptr)
// //     }
// // }
// //
// // impl BindSymbols for FnParam {
// //     fn item_ref(ptr: AstPtr<Self>) -> ItemRef<'static> {
// //         ItemRef::FnParam(ptr)
// //     }
// // }