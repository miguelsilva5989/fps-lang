use std::io;
use std::rc::Rc;

use crate::ast::fps::Fps;
use crate::ast::literal::LiteralValue;
use crate::ast::{environment::Environment, statement::Statement};

use anyhow::Result;

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
    fps: Fps,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            fps: Fps::new(),
        }
    }

    fn interpret_block(&mut self, frame: usize, stdout: &mut dyn io::Write, statements: Vec<Statement>) -> Result<()> {
        for statement in statements {
            match statement {
                Statement::Fps(_) => {}
                Statement::FpsEnd(_) => {}
                Statement::Comment(_) => {}
                Statement::ArithmeticExpr(expr) => {
                    expr.eval(&mut self.environment)?;
                }
                Statement::Print(expr) => {
                    let value = expr.eval(&mut self.environment)?;
                    writeln!(stdout, "FPS {} -> {}", frame, value).unwrap();
                }
                Statement::Declaration { id, expr } => {
                    let value = expr.eval(&mut self.environment)?;
                    self.environment.declare(id.lexeme, value)?;
                    // println!("{value}");
                }
                Statement::Block { statements: block_statements } => {
                    let mut new_env = Environment::new();
                    new_env.parent = Some(Rc::from(self.environment.clone()));
                    // println!("new_env {:?}", new_env);
                    let old_env = self.environment.clone();
                    // println!("old_env {:?}", old_env);
                    // println!("new_env {:?}", new_env);
                    self.environment = new_env;
                    self.interpret_block(frame, stdout, block_statements)?;
                    // println!("old_env {:?}", old_env);
                    self.environment = old_env;

                }
                Statement::If {
                    condition,
                    then_block,
                    else_block,
                } => {
                    let cond = condition.eval(&mut self.environment)?;

                    if cond.is_true()? == LiteralValue::Boolean(true) {
                        self.interpret_block(frame, stdout, then_block)?;
                    } else if let Some(else_block) = else_block {
                        self.interpret_block(frame, stdout, else_block)?;
                    }
                }
                Statement::For { range: _, for_block: _ } => panic!("should no exist a the for block should be destructured into multiple statements"),
                // Statement::For { range, for_block } => {
                //     let range = range.eval(&mut self.environment)?;
                //     let mut for_blocks: Vec<Statement> = vec![];
                //     match range {
                //         LiteralValue::Range((start, end)) => {
                //             let range = start..end;
                //             for _ in range {
                //                 for_blocks.push(*for_block.clone());
                //             }
                //         }
                //         LiteralValue::RangeEqual((start, end)) => {
                //             let range = start..=end;
                //             for _ in range {
                //                 for_blocks.push(*for_block.clone());
                //             }
                //         }
                //         _ => panic!(),
                //     }

                //     self.interpret_block(frame, stdout, for_blocks)?;
                // }
            };
        }
        Ok(())
    }

    pub fn interpret(&mut self, stdout: &mut dyn io::Write, statements: Vec<Statement>) -> Result<()> {
        self.fps.allocate_statements_to_frame(&mut self.environment, statements)?;
        // println!("frames {:?}", self.fps.frames);

        for (frame, range_statements) in self.fps.frames.clone() {
            // println!("frame {} statements: {:?}", frame, range_statements);
            for statement in range_statements {
                // println!("{:?}", statement);
                // match statement {
                //     Statement::For { range, for_block } => todo!(),
                //     _ => self.interpret_block(frame, stdout, vec![statement])?,
                // }

                self.interpret_block(frame, stdout, vec![statement])?
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::FpsInput, parser::Parser};

    use super::*;
    #[test]
    fn multiple_frames() {
        let mut stdout = Vec::new();

        let input = "let a = 1; #2 a = a + 1; # print(a); ##";
        let expected = "FPS 4 -> 3\n";

        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(scanner.tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        interpreter.interpret(&mut stdout, statements).unwrap();

        assert_eq!(std::str::from_utf8(&stdout).unwrap(), expected)
    }
}
