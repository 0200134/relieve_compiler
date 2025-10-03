use crate::types::RelieveType;

// 최상위 프로그램 구조체
#[derive(Debug)]
pub struct Program {
    pub definitions: Vec<Definition>,
}

// 정의 (함수, 구조체 등)
#[derive(Debug)]
pub enum Definition {
    Function(FunctionDefinition),
    // TODO: Structure, Enum definition
}

// 함수 정의 구조체
#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: RelieveType,
    pub body: Block, // Statement block
}

// 블록 구조체
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

// 매개변수 구조체 (E0422 해결)
#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: RelieveType,
}

// 구문 (Statement)
#[derive(Debug)]
pub enum Statement {
    // 필드 구조는 parser.rs와 일치해야 합니다.
    VariableDeclaration {
        name: String,
        is_mutable: bool,           
        type_hint: Option<RelieveType>, // Option<T>으로 변경
        initializer: Expression,
    },
    FunctionCall(FunctionCall),
    // TODO: If, Loop, Return statement
}

// 표현식 (Expression)
#[derive(Debug)]
pub enum Expression {
    Literal(LiteralValue),
    Identifier(String), // 변수 이름
    FunctionCall(FunctionCall), // E0599 FunctionCall 해결
    // TODO: BinaryOp, UnaryOp
}

// 함수 호출 (Function Call)
#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

// 리터럴 값 (Literal Value)
#[derive(Debug)]
pub enum LiteralValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    // TODO: Float, Array
}
