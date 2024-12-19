//! GENERATED CODE -- DO NOT EDIT!

use crate::frontend::syntax::{SyntaxKind, SyntaxNode, SyntaxToken, IXCLanguage};
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
impl crate::frontend::ast::AstStruct for File {
    const KIND: SyntaxKind = SyntaxKind::FILE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {}
impl rowan::ast::AstNode for Item {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
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
impl crate::frontend::ast::AstStruct for Interface {
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
impl crate::frontend::ast::AstStruct for Handler {
    const KIND: SyntaxKind = SyntaxKind::HANDLER;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceItem {}
impl rowan::ast::AstNode for InterfaceItem {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
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
    pub fn sig(&self) -> Option<FnSignature> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for InterfaceFn {
    const KIND: SyntaxKind = SyntaxKind::INTERFACE_FN;
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
impl crate::frontend::ast::AstStruct for Struct {
    const KIND: SyntaxKind = SyntaxKind::STRUCT;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Event {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EVENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Event {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn fields(&self) -> rowan::ast::AstChildren<StructField> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for Event {
    const KIND: SyntaxKind = SyntaxKind::EVENT;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnSignature {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnSignature {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_SIGNATURE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnSignature {
    #[inline]
    pub fn typ(&self) -> Option<FnType> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn args(&self) -> Option<FnParamList> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn events(&self) -> Option<FnEvents> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn ret(&self) -> Option<FnRet> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for FnSignature {
    const KIND: SyntaxKind = SyntaxKind::FN_SIGNATURE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FnType {}
impl rowan::ast::AstNode for FnType {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParamList {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnParamList {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_PARAM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnParamList {
    #[inline]
    pub fn args(&self) -> rowan::ast::AstChildren<FnParam> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for FnParamList {
    const KIND: SyntaxKind = SyntaxKind::FN_PARAM_LIST;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnEvents {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnEvents {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_EVENTS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnEvents {
    #[inline]
    pub fn events(&self) -> rowan::ast::AstChildren<Type> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for FnEvents {
    const KIND: SyntaxKind = SyntaxKind::FN_EVENTS;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnRet {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnRet {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_RET
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnRet {
    #[inline]
    pub fn ret(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::frontend::ast::AstStruct for FnRet {
    const KIND: SyntaxKind = SyntaxKind::FN_RET;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParam {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnParam {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_PARAM
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnParam {
    #[inline]
    pub fn key(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::KEY_KW)
    }
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for FnParam {
    const KIND: SyntaxKind = SyntaxKind::FN_PARAM;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {}
impl rowan::ast::AstNode for Type {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
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
impl crate::frontend::ast::AstStruct for TypeIdent {
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
impl crate::frontend::ast::AstStruct for TypeArray {
    const KIND: SyntaxKind = SyntaxKind::TYPE_ARRAY;
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
impl crate::frontend::ast::AstStruct for StructField {
    const KIND: SyntaxKind = SyntaxKind::STRUCT_FIELD;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapCollection {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for MapCollection {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MAP_COLLECTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MapCollection {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn key_fields(&self) -> Option<MapKeyFields> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn value_fields(&self) -> Option<MapValueFields> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for MapCollection {
    const KIND: SyntaxKind = SyntaxKind::MAP_COLLECTION;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapKeyFields {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for MapKeyFields {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MAP_KEY_FIELDS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MapKeyFields {
    #[inline]
    pub fn fields(&self) -> rowan::ast::AstChildren<MapField> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for MapKeyFields {
    const KIND: SyntaxKind = SyntaxKind::MAP_KEY_FIELDS;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapValueFields {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for MapValueFields {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MAP_VALUE_FIELDS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MapValueFields {
    #[inline]
    pub fn fields(&self) -> rowan::ast::AstChildren<MapField> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for MapValueFields {
    const KIND: SyntaxKind = SyntaxKind::MAP_VALUE_FIELDS;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapField {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for MapField {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MAP_FIELD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MapField {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for MapField {
    const KIND: SyntaxKind = SyntaxKind::MAP_FIELD;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Client {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Client {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLIENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Client {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn types(&self) -> Option<ClientTypes> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for Client {
    const KIND: SyntaxKind = SyntaxKind::CLIENT;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientTypes {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ClientTypes {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLIENT_TYPES
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ClientTypes {
    #[inline]
    pub fn types(&self) -> rowan::ast::AstChildren<ClientType> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ClientTypes {
    const KIND: SyntaxKind = SyntaxKind::CLIENT_TYPES;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientType {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ClientType {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLIENT_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ClientType {
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ClientType {
    const KIND: SyntaxKind = SyntaxKind::CLIENT_TYPE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Impl {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Impl {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Impl {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn for_(&self) -> Option<ImplFor> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn items(&self) -> rowan::ast::AstChildren<ImplItem> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for Impl {
    const KIND: SyntaxKind = SyntaxKind::IMPL;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplFor {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ImplFor {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPL_FOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ImplFor {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::frontend::ast::AstStruct for ImplFor {
    const KIND: SyntaxKind = SyntaxKind::IMPL_FOR;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplItem {}
impl rowan::ast::AstNode for ImplItem {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplFn {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ImplFn {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPL_FN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ImplFn {
    #[inline]
    pub fn sig(&self) -> Option<FnSignature> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn block(&self) -> Option<FnBlock> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ImplFn {
    const KIND: SyntaxKind = SyntaxKind::IMPL_FN;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnBlock {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnBlock {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_BLOCK
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnBlock {
    #[inline]
    pub fn statements(&self) -> rowan::ast::AstChildren<Stmt> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for FnBlock {
    const KIND: SyntaxKind = SyntaxKind::FN_BLOCK;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {}
impl rowan::ast::AstNode for Stmt {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {}
impl rowan::ast::AstNode for Expr {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprParen {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ExprParen {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPR_PAREN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprParen {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ExprParen {
    const KIND: SyntaxKind = SyntaxKind::EXPR_PAREN;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameExpr {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for NameExpr {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAME_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NameExpr {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::frontend::ast::AstStruct for NameExpr {
    const KIND: SyntaxKind = SyntaxKind::NAME_EXPR;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprCall {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ExprCall {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPR_CALL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprCall {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> Option<ArgList> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ExprCall {
    const KIND: SyntaxKind = SyntaxKind::EXPR_CALL;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldExpr {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FieldExpr {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FIELD_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FieldExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::frontend::ast::AstStruct for FieldExpr {
    const KIND: SyntaxKind = SyntaxKind::FIELD_EXPR;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ArgList {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARG_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ArgList {
    #[inline]
    pub fn args(&self) -> rowan::ast::AstChildren<Arg> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ArgList {
    const KIND: SyntaxKind = SyntaxKind::ARG_LIST;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprBinary {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ExprBinary {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPR_BINARY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprBinary {
    #[inline]
    pub fn op(&self) -> Option<BinaryOp> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn exprs(&self) -> rowan::ast::AstChildren<Expr> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ExprBinary {
    const KIND: SyntaxKind = SyntaxKind::EXPR_BINARY;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {}
impl rowan::ast::AstNode for BinaryOp {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        todo!()
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        todo!()
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rhs {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Rhs {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RHS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Rhs {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for Rhs {
    const KIND: SyntaxKind = SyntaxKind::RHS;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arg {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Arg {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARG
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Arg {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for Arg {
    const KIND: SyntaxKind = SyntaxKind::ARG;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprConstruct {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ExprConstruct {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPR_CONSTRUCT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprConstruct {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn fields(&self) -> Option<ExprConstructFieldList> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ExprConstruct {
    const KIND: SyntaxKind = SyntaxKind::EXPR_CONSTRUCT;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprConstructFieldList {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ExprConstructFieldList {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPR_CONSTRUCT_FIELD_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprConstructFieldList {
    #[inline]
    pub fn fields(&self) -> rowan::ast::AstChildren<ExprConstructField> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ExprConstructFieldList {
    const KIND: SyntaxKind = SyntaxKind::EXPR_CONSTRUCT_FIELD_LIST;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprConstructField {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ExprConstructField {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPR_CONSTRUCT_FIELD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprConstructField {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ExprConstructField {
    const KIND: SyntaxKind = SyntaxKind::EXPR_CONSTRUCT_FIELD;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for ForStmt {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FOR_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ForStmt {
    #[inline]
    pub fn pat(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn iterable(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn block(&self) -> Option<FnBlock> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::AstStruct for ForStmt {
    const KIND: SyntaxKind = SyntaxKind::FOR_STMT;
}
