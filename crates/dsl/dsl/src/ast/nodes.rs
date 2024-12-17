//! GENERATED CODE -- DO NOT EDIT!

use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for File {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FILE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl File {
    #[inline]
    pub fn items(&self) -> rowan::ast::AstChildren<Item> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::ast::AstStruct for File {
    const KIND: SyntaxKind = SyntaxKind::FILE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Interface {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Interface {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INTERFACE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Interface {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn items(&self) -> rowan::ast::AstChildren<InterfaceItem> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::ast::AstStruct for Interface {
    const KIND: SyntaxKind = SyntaxKind::INTERFACE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Handler {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Handler {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::HANDLER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Handler {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::ast::AstStruct for Handler {
    const KIND: SyntaxKind = SyntaxKind::HANDLER;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceFn {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for InterfaceFn {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INTERFACE_FN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl InterfaceFn {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn args(&self) -> rowan::ast::AstChildren<FnArg> {
        rowan::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn ret(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::ast::AstStruct for InterfaceFn {
    const KIND: SyntaxKind = SyntaxKind::INTERFACE_FN;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnArg {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnArg {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_ARG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnArg {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstStruct for FnArg {
    const KIND: SyntaxKind = SyntaxKind::FN_ARG;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeIdent {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for TypeIdent {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TYPE_IDENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeIdent {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::ast::AstStruct for TypeIdent {
    const KIND: SyntaxKind = SyntaxKind::TYPE_IDENT;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArray {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for TypeArray {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TYPE_ARRAY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeArray {
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstStruct for TypeArray {
    const KIND: SyntaxKind = SyntaxKind::TYPE_ARRAY;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Struct {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRUCT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Struct {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn fields(&self) -> rowan::ast::AstChildren<StructField> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::ast::AstStruct for Struct {
    const KIND: SyntaxKind = SyntaxKind::STRUCT;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructField {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for StructField {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRUCT_FIELD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StructField {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstStruct for StructField {
    const KIND: SyntaxKind = SyntaxKind::STRUCT_FIELD;
}
