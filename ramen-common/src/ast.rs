use std::sync::atomic::AtomicUsize;

use crate::Loc;

static _NEXT_NODE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct NodeId(usize);

impl NodeId {
    pub fn next() -> Self {
        Self(_NEXT_NODE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID:{}", self.0)
    }
}

#[derive(Debug)]
pub struct Item {
    pub location: Loc,
    pub attributes: Attributes,
    pub kind: ItemKind,
    pub id: NodeId,
}

#[derive(Debug)]
pub enum ItemKind {
    Module(Module),

    Function(Function)
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Option<Type>,
    pub parameters: Vec<ValueParameter>,
    pub body: Block
}

#[derive(Debug)]
pub struct Block {
    pub location: Loc,
    pub statements: Vec<Statement>,
    pub id: NodeId,
}

#[derive(Debug)]
pub struct Statement {
    pub location: Loc,
    pub kind: StatementKind,
    pub id: NodeId,
}

#[derive(Debug)]
pub enum StatementKind {
    Item(Item),
    Expression(Expression)
}

#[derive(Debug)]
pub struct Expression {
    pub location: Loc,
    pub kind: ExpressionKind,
    pub id: NodeId,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Literal(Literal)
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(usize)
}

#[derive(Debug)]
pub struct ValueParameter {
    pub location: Loc,
    pub parameter: Parameter,
    pub initializer: Option<Expression>,
    pub id: NodeId,
}

#[derive(Debug)]
pub struct Parameter {
    pub location: Loc,
    pub name: String,
    pub ty: Type,
    pub id: NodeId,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub location: Loc,
    pub kind: TypeKind,
    pub id: NodeId,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Unit,
    /// Integer type of arbitrary width
    Integer(usize)
}

#[derive(Debug, Clone)]
pub struct Attributes(pub Vec<Attribute>);

#[derive(Debug, Clone)]
pub struct Attribute {
    pub location: Loc,
    pub kind: AttributeKind,
    pub id: NodeId,
}

#[derive(Debug, Clone)]
pub enum AttributeKind {
    Marker(String)
}