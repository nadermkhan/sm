extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use std::ffi::CString;

#[derive(Parser)]
#[grammar = "parser/lexer.pest"]
pub struct OParser;

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    Div(Box<AstNode>, Box<AstNode>),
    Assign(i32, String, Box<AstNode>),
    Print(Box<AstNode>),
    Ident(String),
    Str(CString),
    Terms(Vec<AstNode>),
    Integer(i64),
    DoublePrecisionFloat(f64),
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum NumAstNode {
    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    Div(Box<AstNode>, Box<AstNode>),
    Assign(i32, String, Box<AstNode>),
    Ident(String),
    Terms(Vec<AstNode>),
    Integer(i64),
    DoublePrecisionFloat(f64),
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum StrAstNode {
    Add(Box<AstNode>, Box<AstNode>),
    Assign(i32, String, Box<AstNode>),
    Ident(String),
    Terms(Vec<AstNode>),
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DyadicVerb {
    Plus,
    Times,
    // LessThan,
    // LargerThan,
    // Equal,
    Minus,
    Divide,
    // Power,
    // Residue,
    // Copy,
    // LargerOf,
    // LargerOrEqual,
    // Shape,
}

fn get_pairs(result: Result<pest::iterators::Pairs<'_, Rule>, pest::error::Error<Rule>>)
    -> Option<pest::iterators::Pairs<'_, Rule>> {
    match result {
        Ok(pairs) => {
            return Some(pairs);
        },
        Err(e) => {
            println!("error: {:?}", e);
            return None;
        },
    }
}

pub fn parse(source: &str) -> Result<Vec<Box<AstNode>>, Error<Rule>> {
    let mut ast = vec![];

    let result = OParser::parse(Rule::program, source);
    let pairs = get_pairs(result);
    if pairs != None {
        for pair in pairs {
            // A pair can be converted to an iterator of the tokens which make it up:
            for inner_pair in pair {
                match inner_pair.as_rule() {
                    Rule::expr => {
                        for expr in inner_pair.into_inner() {
                            ast.push(Box::new(build_ast_from_expr(expr)));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(ast)
}

fn main() {
    use std::collections::HashMap;
    let s = "
    //4*2+4+5*2;
    let test1 = 4*2+4+5*2;
    //const test2 = 2;
    //test1 + test2;
    //const test = 5+5*10;
    //let str = 'a'; //aaaaaaaaaauhiih dfgtdt
    //str = 'abc'; //aaabbbccc";
    let astnode = parse(&s).expect("unsuccessful parse");
    println!("---{:?}---", astnode);
    let mut env = HashMap::new();
    for reduced_expr in &astnode {
        //println!("{:?}", reduced_expr);
        match reduced_expr {
            // AstNode::Ident(ref var) => {
            //     let v = *env.get(&var[..]).unwrap();
            //     v as f64
            // },
            // AstNode::Assign(ref var_type, ref ident, ref expr) => {
            //     let val = num_interp_expr(env, expr);
            //     env.insert(ident, val);
            //     val
            // }
            // AstNode::Str(cstr) => cstr,
            _ => num_interp_expr(&mut env, reduced_expr)
        };
    }

    fn num_interp_expr<'a>(env: &mut HashMap<&'a str, f64>, reduced_expr: &'a AstNode) -> f64 {
        match reduced_expr {
            AstNode::Terms(ref term) => {
                println!("term:{:?}", term);
                0 as f64
            }
            AstNode::DyadicOp {ref verb, ref lhs, ref rhs } => {
                match verb {
                    DyadicVerb::Plus => { 
                        let test = num_interp_expr(env, lhs) + num_interp_expr(env, rhs);
                        println!("{}", test);
                        test
                    }
                    DyadicVerb::Minus => { num_interp_expr(env, lhs) - num_interp_expr(env, rhs) }
                    DyadicVerb::Times => { 
                        let test2 = num_interp_expr(env, lhs) * num_interp_expr(env, rhs);
                        println!("{}", test2);
                        test2
                    }
                    DyadicVerb::Divide => { 
                        let test3 = num_interp_expr(env, lhs) / num_interp_expr(env, rhs);
                        println!("{}", test3);
                        test3
                    }
                }
            },
            AstNode::DoublePrecisionFloat(double) => *double,
            AstNode::Integer(integer) => *integer as f64,
            AstNode::Ident(ref var) => {
                let v = *env.get(&var[..]).unwrap();
                v as f64
            },
            AstNode::Assign(ref var_type, ref ident, ref expr) => {
                let val = num_interp_expr(env, expr);
                env.insert(ident, val);
                val
            }
            _ => {
                //println!("{:?}", reduced_expr);
                1.0
            }
        }
    }
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::dyadicExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let lhs = build_ast_from_expr(lhspair);
            let verb = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let rhs = build_ast_from_expr(rhspair);
            parse_dyadic_verb(verb, lhs, rhs)
        }
        Rule::string => {
            let str = &pair.as_str();
            // Strip leading and ending quotes.
            let str = &str[1..str.len() - 1];
            // Escaped string quotes become single quotes here.
            let str = str.replace("''", "'");
            AstNode::Str(CString::new(&str[..]).unwrap())
        }
        Rule::assgmtExpr => {
            let mut pair = pair.into_inner();
            let var_prefix = pair.next().unwrap();
            let var_type = match var_prefix.as_str() {
                "const" => 0, // const
                "let" => 1, // let
                _ => panic!("unknown variable type: {:?}", var_prefix)
            };
            let ident = pair.next().unwrap();
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            AstNode::Assign (
                var_type,
                String::from(ident.as_str()),
                Box::new(expr),
            )
        }
        Rule::reAssgmtExpr => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            AstNode::Assign (
                2, //re-assign
                String::from(ident.as_str()),
                Box::new(expr),
            )
        }
        Rule::terms => {
            let terms: Vec<AstNode> = pair.into_inner().map(build_ast_from_term).collect();
            // If there's just a single term, return it without
            // wrapping it in a Terms node.
            match terms.len() {
                1 => terms.get(0).unwrap().clone(),
                _ => AstNode::Terms(terms),
            }
        }
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}

fn build_ast_from_term(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => {
            let istr = pair.as_str();
            let integer: i64 = istr.parse().unwrap();
            AstNode::Integer(integer)
        }
        Rule::decimal => {
            let dstr = pair.as_str();
            let flt: f64 = dstr.parse().unwrap();
            AstNode::DoublePrecisionFloat(flt)
        }
        Rule::expr => build_ast_from_expr(pair),
        Rule::ident => AstNode::Ident(String::from(pair.as_str())),
        unknown_term => panic!("Unexpected term: {:?}", unknown_term),
    }
}

fn parse_dyadic_verb(pair: Pair<Rule>, lhs: AstNode, rhs: AstNode) -> AstNode {
    AstNode::DyadicOp {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        verb: match pair.as_str() {
            "+" => DyadicVerb::Plus,
            "*" => DyadicVerb::Times,
            "-" => DyadicVerb::Minus,
            "/" => DyadicVerb::Divide,
            _ => panic!("Unexpected dyadic verb: {}", pair.as_str()),
        },
    }
}
