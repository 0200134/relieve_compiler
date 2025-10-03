use pest::Parser;
use pest_derive::Parser;

use crate::ast::*;
use crate::types::RelieveType;
use self::Rule;
#[derive(Parser)]
#[grammar = "relieve.pest"]
pub struct RelieveParser;



/// 파서 헬퍼 구조체 (사용자가 직접 만든 것)
pub struct RelieveAstBuilder;

impl RelieveAstBuilder {
    pub fn new() -> Self {
        RelieveAstBuilder
    }

    pub fn parse_program(&self, source: &str) -> Result<Program, String> {
        let pairs = RelieveParser::parse(Rule::program, source)
            .map_err(|e| format!("Parsing failed:\n{}", e))?;

        let mut program = Program {
            definitions: Vec::new(),
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => (),
                Rule::function_definition => {
                    if let Ok(func_def) = self.parse_function_definition(pair) {
                        program.definitions.push(Definition::Function(func_def));
                    }
                }
                _ => return Err(format!("Unexpected rule at top level: {:?}", pair.as_rule())),
            }
        }

        Ok(program)
    }

    fn parse_function_definition(&self, pair: pest::iterators::Pair<Rule>) -> Result<FunctionDefinition, String> {
        let mut inner_rules = pair.into_inner();

        let name_pair = inner_rules.next().unwrap();
        let name = name_pair.as_str().to_string();

        // 파라미터
        let mut parameters = Vec::new();
        if let Some(param_list) = inner_rules.next() {
            for param_pair in param_list.into_inner() {
                if param_pair.as_rule() == Rule::parameter {
                    let mut param_inner = param_pair.into_inner();
                    let type_name_pair = param_inner.next().unwrap();
                    let param_type = self.parse_type_name(type_name_pair.as_str())?;
                    let param_name = param_inner.next().unwrap().as_str().to_string();

                    parameters.push(Parameter { name: param_name, param_type });
                }
            }
        }

        // 리턴 타입
        let return_type_pair = inner_rules.next().unwrap();
        let return_type = self.parse_type_name(return_type_pair.as_str())?;

        // 블록
        let block_pair = inner_rules.next().unwrap();
        let body = self.parse_block(block_pair)?;

        Ok(FunctionDefinition { name, parameters, return_type, body })
    }

    fn parse_block(&self, pair: pest::iterators::Pair<Rule>) -> Result<Block, String> {
        let mut statements = Vec::new();
        for statement_pair in pair.into_inner() {
            if statement_pair.as_rule() == Rule::statement {
                statements.push(self.parse_statement(statement_pair)?);
            }
        }
        Ok(Block { statements })
    }

    fn parse_statement(&self, pair: pest::iterators::Pair<Rule>) -> Result<Statement, String> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::variable_declaration => {
                let mut decl_inner = inner.into_inner();
                let name = decl_inner.next().unwrap().as_str().to_string();
                let type_name_pair = decl_inner.next().unwrap();
                let type_hint = self.parse_type_name(type_name_pair.as_str())?;
                let initializer = self.parse_expression(decl_inner.next().unwrap())?;

                Ok(Statement::VariableDeclaration {
                    name,
                    is_mutable: false,
                    type_hint: Some(type_hint),
                    initializer,
                })
            }
            Rule::expression => {
                let expression_inner = inner.into_inner().next().unwrap();
                match expression_inner.as_rule() {
                    Rule::function_call => Ok(Statement::FunctionCall(self.parse_function_call(expression_inner)?)),
                    _ => Err(format!("Unsupported statement expression: {:?}", expression_inner.as_rule())),
                }
            }
            _ => Err(format!("Unsupported statement: {:?}", inner.as_rule())),
        }
    }

    fn parse_expression(&self, pair: pest::iterators::Pair<Rule>) -> Result<Expression, String> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::literal => self.parse_literal(inner),
            Rule::identifier => Ok(Expression::Identifier(inner.as_str().to_string())),
            Rule::function_call => Ok(Expression::FunctionCall(self.parse_function_call(inner)?)),
            _ => Err(format!("Unsupported expression: {:?}", inner.as_rule())),
        }
    }

    fn parse_function_call(&self, pair: pest::iterators::Pair<Rule>) -> Result<FunctionCall, String> {
        let mut inner_rules = pair.into_inner();
        let name = inner_rules.next().unwrap().as_str().to_string();

        let mut args = Vec::new();
        if let Some(arguments_pair) = inner_rules.next() {
            for arg in arguments_pair.into_inner() {
                args.push(self.parse_expression(arg)?);
            }
        }

        Ok(FunctionCall { name, arguments: args })
    }

    fn parse_literal(&self, pair: pest::iterators::Pair<Rule>) -> Result<Expression, String> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::string_literal => Ok(Expression::Literal(LiteralValue::String(inner.as_str().trim_matches('"').to_string()))),
            Rule::integer_literal => inner.as_str().parse::<i64>()
                .map(|i| Expression::Literal(LiteralValue::Integer(i)))
                .map_err(|_| format!("Invalid integer literal: {}", inner.as_str())),
            Rule::boolean_literal => {
                let b = inner.as_str() == "true";
                Ok(Expression::Literal(LiteralValue::Boolean(b)))
            }
            _ => Err(format!("Unsupported literal: {:?}", inner.as_rule())),
        }
    }

    fn parse_type_name(&self, type_name: &str) -> Result<RelieveType, String> {
        match type_name {
            "Int" => Ok(RelieveType::Integer),
            "String" => Ok(RelieveType::String),
            "Bool" => Ok(RelieveType::Boolean),
            "Res" => Ok(RelieveType::Unit),
            _ => Err(format!("Unknown type: {}", type_name)),
        }
    }
}
