// Relieve 언어의 타입 시스템을 정의합니다.
#[derive(Debug, PartialEq, Clone)]
pub enum RelieveType {
 // 기본 타입
String,
Integer,
Boolean,

// 특수 타입
 Unit, // 아무것도 반환하지 않음을 나타내는 타입 (Rust의 ()) - E0599 해결
Unknown, // 타입 추론이 안되거나 임시적으로 사용되는 타입

// TODO:
// Array(Box<RelieveType>),
 // Function { args: Vec<RelieveType>, return_type: Box<RelieveType> }
}
