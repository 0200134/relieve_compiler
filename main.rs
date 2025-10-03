use std::fs;

// --- 모듈 선언 ---
pub mod parser;
pub mod codegen;
pub mod ast;
pub mod types;

// --- 모듈 사용 ---
// E0432: unresolved import `parser::Parser` 오류 해결
use crate::parser::Parser;
use crate::codegen::Codegen;

// Define the relative path to the source file
const RELIEVE_SOURCE_FILE: &str = "main.relieve";

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Relieve 코드 파싱 시작 ---");
    
    // 1. Read the source file
    let source_code = fs::read_to_string(RELIEVE_SOURCE_FILE)
        .map_err(|e| format!("Error reading file {}: {}", RELIEVE_SOURCE_FILE, e))?;

    // 2. Initialize the parser and parse the code
    let parser = Parser::new();
    let ast_program = match parser.parse_program(&source_code) {
        Ok(ast) => {
            println!("✅ Relieve 코드 파싱 및 AST 변환 성공!");
            ast
        },
        Err(e) => {
            // Print the detailed parsing error
            return Err(format!("파싱 오류 발생: {}", e).into());
        }
    };

    // 3. Print the generated AST (for debugging)
    println!("--- 생성된 AST 구조 (일부) ---");
    // Since we only expect one function for now, we process the first definition
    if !ast_program.definitions.is_empty() {
        let ast::Definition::Function(func_def) = &ast_program.definitions[0];
        println!("함수 이름: {}", func_def.name);
        println!("반환 타입: {:?}", func_def.return_type);
        println!("본문 구문: {:?}", func_def.body);
    }
    
    // 4. Generate Rust code from AST
    println!("\n--- Rust 코드 생성 시작 ---");
    // E0599: new 함수 호출 오류 해결 (src/codegen.rs의 new 함수는 이미 pub으로 정의됨)
    let codegen = Codegen::new();
    let generated_code = codegen.generate_program(&ast_program);
    
    // 5. Print the generated code
    println!("✅ 코드 생성 성공!");
    println!("\n--- 최종 생성된 Rust 코드 ---\n");
    println!("{}", generated_code);
    
    // Optional: Write the generated code to an output file
    // fs::write("target/generated_output.rs", &generated_code)?;
    
    Ok(())
}
