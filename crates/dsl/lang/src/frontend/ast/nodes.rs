//! GENERATED CODE -- DO NOT EDIT!

use crate::frontend::syntax::{SyntaxKind, SyntaxNode, SyntaxToken, IXCLanguage};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Name {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Name {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::frontend::ast::ConcreteNode for Name {
    const KIND: SyntaxKind = SyntaxKind::NAME;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for NameRef {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAME_REF
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NameRef {
    #[inline]
    pub fn name_ref(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
}
impl crate::frontend::ast::ConcreteNode for NameRef {
    const KIND: SyntaxKind = SyntaxKind::NAME_REF;
}
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
impl crate::frontend::ast::ConcreteNode for File {
    const KIND: SyntaxKind = SyntaxKind::FILE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Interface(Interface),
    Object(Object),
    Impl(Impl),
    Test(Test),
}
impl rowan::ast::AstNode for Item {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind, SyntaxKind::INTERFACE | SyntaxKind::OBJECT | SyntaxKind::IMPL |
            SyntaxKind::TEST
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::INTERFACE => Item::Interface(Interface { syntax }),
            SyntaxKind::OBJECT => Item::Object(Object { syntax }),
            SyntaxKind::IMPL => Item::Impl(Impl { syntax }),
            SyntaxKind::TEST => Item::Test(Test { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Item::Interface(it) => &it.syntax,
            Item::Object(it) => &it.syntax,
            Item::Impl(it) => &it.syntax,
            Item::Test(it) => &it.syntax,
        }
    }
}
impl From<Interface> for Item {
    fn from(node: Interface) -> Self {
        Self::Interface(node)
    }
}
impl From<Object> for Item {
    fn from(node: Object) -> Self {
        Self::Object(node)
    }
}
impl From<Impl> for Item {
    fn from(node: Impl) -> Self {
        Self::Impl(node)
    }
}
impl From<Test> for Item {
    fn from(node: Test) -> Self {
        Self::Test(node)
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn items(&self) -> rowan::ast::AstChildren<InterfaceItem> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for Interface {
    const KIND: SyntaxKind = SyntaxKind::INTERFACE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Object {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Object {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OBJECT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Object {
    #[inline]
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn items(&self) -> rowan::ast::AstChildren<ObjectItem> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for Object {
    const KIND: SyntaxKind = SyntaxKind::OBJECT;
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
    pub fn name_ref(&self) -> Option<NameRef> {
        rowan::ast::support::child(&self.syntax)
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
impl crate::frontend::ast::ConcreteNode for Impl {
    const KIND: SyntaxKind = SyntaxKind::IMPL;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Test {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for Test {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TEST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Test {
    #[inline]
    pub fn name(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn block(&self) -> Option<FnBlock> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for Test {
    const KIND: SyntaxKind = SyntaxKind::TEST;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterfaceItem {
    InterfaceFn(InterfaceFn),
    Struct(Struct),
    Event(Event),
    MapCollection(MapCollection),
    VarCollection(VarCollection),
}
impl rowan::ast::AstNode for InterfaceItem {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind, SyntaxKind::INTERFACE_FN | SyntaxKind::STRUCT | SyntaxKind::EVENT |
            SyntaxKind::MAP_COLLECTION | SyntaxKind::VAR_COLLECTION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::INTERFACE_FN => {
                InterfaceItem::InterfaceFn(InterfaceFn { syntax })
            }
            SyntaxKind::STRUCT => InterfaceItem::Struct(Struct { syntax }),
            SyntaxKind::EVENT => InterfaceItem::Event(Event { syntax }),
            SyntaxKind::MAP_COLLECTION => {
                InterfaceItem::MapCollection(MapCollection { syntax })
            }
            SyntaxKind::VAR_COLLECTION => {
                InterfaceItem::VarCollection(VarCollection { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            InterfaceItem::InterfaceFn(it) => &it.syntax,
            InterfaceItem::Struct(it) => &it.syntax,
            InterfaceItem::Event(it) => &it.syntax,
            InterfaceItem::MapCollection(it) => &it.syntax,
            InterfaceItem::VarCollection(it) => &it.syntax,
        }
    }
}
impl From<InterfaceFn> for InterfaceItem {
    fn from(node: InterfaceFn) -> Self {
        Self::InterfaceFn(node)
    }
}
impl From<Struct> for InterfaceItem {
    fn from(node: Struct) -> Self {
        Self::Struct(node)
    }
}
impl From<Event> for InterfaceItem {
    fn from(node: Event) -> Self {
        Self::Event(node)
    }
}
impl From<MapCollection> for InterfaceItem {
    fn from(node: MapCollection) -> Self {
        Self::MapCollection(node)
    }
}
impl From<VarCollection> for InterfaceItem {
    fn from(node: VarCollection) -> Self {
        Self::VarCollection(node)
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
impl crate::frontend::ast::ConcreteNode for InterfaceFn {
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn fields(&self) -> rowan::ast::AstChildren<StructField> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for Struct {
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn fields(&self) -> rowan::ast::AstChildren<StructField> {
        rowan::ast::support::children(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for Event {
    const KIND: SyntaxKind = SyntaxKind::EVENT;
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
    pub fn scoped(&self) -> Option<SyntaxToken> {
        rowan::ast::support::token(&self.syntax, SyntaxKind::ACCOUNT_SCOPED_KW)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
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
impl crate::frontend::ast::ConcreteNode for MapCollection {
    const KIND: SyntaxKind = SyntaxKind::MAP_COLLECTION;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarCollection {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for VarCollection {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VAR_COLLECTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl VarCollection {
    #[inline]
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for VarCollection {
    const KIND: SyntaxKind = SyntaxKind::VAR_COLLECTION;
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
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
impl crate::frontend::ast::ConcreteNode for FnSignature {
    const KIND: SyntaxKind = SyntaxKind::FN_SIGNATURE;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectItem {
    MapCollection(MapCollection),
    VarCollection(VarCollection),
    Client(Client),
    ImplFn(ImplFn),
}
impl rowan::ast::AstNode for ObjectItem {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind, SyntaxKind::MAP_COLLECTION | SyntaxKind::VAR_COLLECTION |
            SyntaxKind::CLIENT | SyntaxKind::IMPL_FN
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::MAP_COLLECTION => {
                ObjectItem::MapCollection(MapCollection { syntax })
            }
            SyntaxKind::VAR_COLLECTION => {
                ObjectItem::VarCollection(VarCollection { syntax })
            }
            SyntaxKind::CLIENT => ObjectItem::Client(Client { syntax }),
            SyntaxKind::IMPL_FN => ObjectItem::ImplFn(ImplFn { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ObjectItem::MapCollection(it) => &it.syntax,
            ObjectItem::VarCollection(it) => &it.syntax,
            ObjectItem::Client(it) => &it.syntax,
            ObjectItem::ImplFn(it) => &it.syntax,
        }
    }
}
impl From<MapCollection> for ObjectItem {
    fn from(node: MapCollection) -> Self {
        Self::MapCollection(node)
    }
}
impl From<VarCollection> for ObjectItem {
    fn from(node: VarCollection) -> Self {
        Self::VarCollection(node)
    }
}
impl From<Client> for ObjectItem {
    fn from(node: Client) -> Self {
        Self::Client(node)
    }
}
impl From<ImplFn> for ObjectItem {
    fn from(node: ImplFn) -> Self {
        Self::ImplFn(node)
    }
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn types(&self) -> Option<ClientTypes> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for Client {
    const KIND: SyntaxKind = SyntaxKind::CLIENT;
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
impl crate::frontend::ast::ConcreteNode for ImplFn {
    const KIND: SyntaxKind = SyntaxKind::IMPL_FN;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnType {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnType {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
impl crate::frontend::ast::ConcreteNode for FnType {
    const KIND: SyntaxKind = SyntaxKind::FN_TYPE;
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
impl crate::frontend::ast::ConcreteNode for FnParamList {
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
impl crate::frontend::ast::ConcreteNode for FnEvents {
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
    pub fn ret(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for FnRet {
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn modifier(&self) -> Option<FnParamModifier> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for FnParam {
    const KIND: SyntaxKind = SyntaxKind::FN_PARAM;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParamModifier {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FnParamModifier {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_PARAM_MODIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
impl crate::frontend::ast::ConcreteNode for FnParamModifier {
    const KIND: SyntaxKind = SyntaxKind::FN_PARAM_MODIFIER;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    TypeIdent(TypeIdent),
    TypeArray(TypeArray),
}
impl rowan::ast::AstNode for Type {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::TYPE_IDENT | SyntaxKind::TYPE_ARRAY)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::TYPE_IDENT => Type::TypeIdent(TypeIdent { syntax }),
            SyntaxKind::TYPE_ARRAY => Type::TypeArray(TypeArray { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Type::TypeIdent(it) => &it.syntax,
            Type::TypeArray(it) => &it.syntax,
        }
    }
}
impl From<TypeIdent> for Type {
    fn from(node: TypeIdent) -> Self {
        Self::TypeIdent(node)
    }
}
impl From<TypeArray> for Type {
    fn from(node: TypeArray) -> Self {
        Self::TypeArray(node)
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
    pub fn name(&self) -> Option<NameRef> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for TypeIdent {
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
impl crate::frontend::ast::ConcreteNode for TypeArray {
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for StructField {
    const KIND: SyntaxKind = SyntaxKind::STRUCT_FIELD;
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
impl crate::frontend::ast::ConcreteNode for MapKeyFields {
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
impl crate::frontend::ast::ConcreteNode for MapValueFields {
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn typ(&self) -> Option<Type> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for MapField {
    const KIND: SyntaxKind = SyntaxKind::MAP_FIELD;
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
impl crate::frontend::ast::ConcreteNode for ClientTypes {
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
impl crate::frontend::ast::ConcreteNode for ClientType {
    const KIND: SyntaxKind = SyntaxKind::CLIENT_TYPE;
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
    pub fn name_ref(&self) -> Option<NameRef> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for ImplFor {
    const KIND: SyntaxKind = SyntaxKind::IMPL_FOR;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplItem {
    ImplFn(ImplFn),
    MapCollection(MapCollection),
    VarCollection(VarCollection),
}
impl rowan::ast::AstNode for ImplItem {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind, SyntaxKind::IMPL_FN | SyntaxKind::MAP_COLLECTION |
            SyntaxKind::VAR_COLLECTION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::IMPL_FN => ImplItem::ImplFn(ImplFn { syntax }),
            SyntaxKind::MAP_COLLECTION => {
                ImplItem::MapCollection(MapCollection { syntax })
            }
            SyntaxKind::VAR_COLLECTION => {
                ImplItem::VarCollection(VarCollection { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ImplItem::ImplFn(it) => &it.syntax,
            ImplItem::MapCollection(it) => &it.syntax,
            ImplItem::VarCollection(it) => &it.syntax,
        }
    }
}
impl From<ImplFn> for ImplItem {
    fn from(node: ImplFn) -> Self {
        Self::ImplFn(node)
    }
}
impl From<MapCollection> for ImplItem {
    fn from(node: MapCollection) -> Self {
        Self::MapCollection(node)
    }
}
impl From<VarCollection> for ImplItem {
    fn from(node: VarCollection) -> Self {
        Self::VarCollection(node)
    }
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
impl crate::frontend::ast::ConcreteNode for FnBlock {
    const KIND: SyntaxKind = SyntaxKind::FN_BLOCK;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    StmtExpr(StmtExpr),
}
impl rowan::ast::AstNode for Stmt {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::STMT_EXPR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::STMT_EXPR => Stmt::StmtExpr(StmtExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::StmtExpr(it) => &it.syntax,
        }
    }
}
impl From<StmtExpr> for Stmt {
    fn from(node: StmtExpr) -> Self {
        Self::StmtExpr(node)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtExpr {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for StmtExpr {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STMT_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StmtExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for StmtExpr {
    const KIND: SyntaxKind = SyntaxKind::STMT_EXPR;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    ExprParen(ExprParen),
    NameExpr(NameExpr),
    ExprCall(ExprCall),
}
impl rowan::ast::AstNode for Expr {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind, SyntaxKind::EXPR_PAREN | SyntaxKind::NAME_EXPR | SyntaxKind::EXPR_CALL
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::EXPR_PAREN => Expr::ExprParen(ExprParen { syntax }),
            SyntaxKind::NAME_EXPR => Expr::NameExpr(NameExpr { syntax }),
            SyntaxKind::EXPR_CALL => Expr::ExprCall(ExprCall { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::ExprParen(it) => &it.syntax,
            Expr::NameExpr(it) => &it.syntax,
            Expr::ExprCall(it) => &it.syntax,
        }
    }
}
impl From<ExprParen> for Expr {
    fn from(node: ExprParen) -> Self {
        Self::ExprParen(node)
    }
}
impl From<NameExpr> for Expr {
    fn from(node: NameExpr) -> Self {
        Self::NameExpr(node)
    }
}
impl From<ExprCall> for Expr {
    fn from(node: ExprCall) -> Self {
        Self::ExprCall(node)
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
impl crate::frontend::ast::ConcreteNode for ExprParen {
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
    pub fn name_ref(&self) -> Option<NameRef> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for NameExpr {
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
impl crate::frontend::ast::ConcreteNode for ExprCall {
    const KIND: SyntaxKind = SyntaxKind::EXPR_CALL;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldRefExpr {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for FieldRefExpr {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FIELD_REF_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FieldRefExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<NameRef> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for FieldRefExpr {
    const KIND: SyntaxKind = SyntaxKind::FIELD_REF_EXPR;
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
impl crate::frontend::ast::ConcreteNode for ArgList {
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
impl crate::frontend::ast::ConcreteNode for ExprBinary {
    const KIND: SyntaxKind = SyntaxKind::EXPR_BINARY;
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryOp {
    syntax: SyntaxNode,
}
impl rowan::ast::AstNode for BinaryOp {
    type Language = IXCLanguage;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BINARY_OP
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
impl crate::frontend::ast::ConcreteNode for BinaryOp {
    const KIND: SyntaxKind = SyntaxKind::BINARY_OP;
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
impl crate::frontend::ast::ConcreteNode for Rhs {
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
    pub fn modifier(&self) -> Option<FnParamModifier> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for Arg {
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn fields(&self) -> Option<ExprConstructFieldList> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for ExprConstruct {
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
impl crate::frontend::ast::ConcreteNode for ExprConstructFieldList {
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
    pub fn name(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        rowan::ast::support::child(&self.syntax)
    }
}
impl crate::frontend::ast::ConcreteNode for ExprConstructField {
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
    pub fn pat(&self) -> Option<Name> {
        rowan::ast::support::child(&self.syntax)
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
impl crate::frontend::ast::ConcreteNode for ForStmt {
    const KIND: SyntaxKind = SyntaxKind::FOR_STMT;
}
