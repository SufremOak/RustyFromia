use clap::Command;
use std::io::{Read, Write};
use pest::Parser;
use pest_derive::Parser;
use crate::RustyFromia::sdk::objctive_r::rsobject::RSObject; // Corrected module path
use crate::RustyFromia::sdk::objctive_r::invoker::Invoker; // Corrected module path

#[derive(Parser)]
#[grammar = "sdk/objctive-r/grammar.pest"]
struct ObjectiveRParser;

fn main() {
    let parser = ObjectiveRParser::parse(Rule::Program, "your code here")
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    // Process the parsed syntax tree
    for stmt in parser.into_inner() {
        match stmt.as_rule() {
            Rule::UseStmt => {
                let path = stmt.into_inner().next().unwrap().as_str();
                println!("Using library: {}", path);
            }
            Rule::InterfaceDef => {
                let mut inner_rules = stmt.into_inner();
                let interface_name = inner_rules.next().unwrap().as_str();
                let base_class = inner_rules.next().unwrap().as_str();
                println!("Defining interface: {} extends {}", interface_name, base_class);
                for method in inner_rules {
                    handle_method_def(method);
                }
            }
            Rule::MethodDef => {
                handle_method_def(stmt);
            }
            Rule::Expr => {
                handle_expr(stmt);
            }
            _ => {}
        }
    }
}

fn handle_method_def(method: pest::iterators::Pair<Rule>) {
    let mut inner_rules = method.into_inner();
    let method_name = inner_rules.next().unwrap().as_str();
    let params = inner_rules.next().unwrap().into_inner().map(|p| p.as_str()).collect::<Vec<_>>();
    println!("Defining method: {} with params {:?}", method_name, params);
    for stmt in inner_rules {
        handle_expr(stmt);
    }
}

fn handle_expr(expr: pest::iterators::Pair<Rule>) {
    match expr.as_rule() {
        Rule::AssignExpr => {
            let mut inner_rules = expr.into_inner();
            let var_name = inner_rules.next().unwrap().as_str();
            let value = inner_rules.next().unwrap();
            println!("Assigning to variable: {}", var_name);
            handle_expr(value);
        }
        Rule::UnaryExpr => {
            let mut inner_rules = expr.into_inner();
            let operator = inner_rules.next().unwrap().as_str();
            let term = inner_rules.next().unwrap();
            println!("Unary expression: {}{}", operator, term.as_str());
            handle_expr(term);
        }
        Rule::BinaryExpr => {
            let mut inner_rules = expr.into_inner();
            let left = inner_rules.next().unwrap();
            let operator = inner_rules.next().unwrap().as_str();
            let right = inner_rules.next().unwrap();
            println!("Binary expression: {} {} {}", left.as_str(), operator, right.as_str());
            handle_expr(left);
            handle_expr(right);
        }
        Rule::Term => {
            let term = expr.as_str();
            println!("Term: {}", term);
        }
        _ => {}
    }
}
