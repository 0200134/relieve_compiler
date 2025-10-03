// lib.rs: 크레이트 루트
use pest_derive::Parser;
use pest::Parser;

// Grammar 파일로부터 Rule enum과 RelieveParser 구조체를 생성합니다.
// 이들은 'crate::Rule', 'crate::RelieveParser'가 되어 E0432를 해결합니다.
#[derive(Parser)]
#[grammar = "relieve.pest"]
pub struct RelieveParser;

// 이 파일이 main.rs 및 그 서브 모듈에서 사용할 모듈을 외부에 노출합니다.
pub mod parser;
pub mod ast;
pub mod types;
pub mod codegen;

