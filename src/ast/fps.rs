use anyhow::Result;
use std::{collections::BTreeMap, ops::Range};

use super::{environment::Environment, statement::Statement};
use crate::ast::LiteralValue as AstLiteralValue;
use crate::lexer::{LiteralValue, Token};

#[derive(Debug)]
pub struct Fps {
    pub frames: BTreeMap<usize, Vec<Statement>>,
    current_range: Range<usize>,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            frames: BTreeMap::new(),
            current_range: 0..1,
        }
    }

    fn get_fps_duration_from_token(&self, token: &Token) -> usize {
        match &token.literal {
            Some(fps) => match fps {
                &LiteralValue::Fps(x) => x,
                _ => panic!("Unexpected literal for FPS: {}", fps),
            },
            None => 0,
        }
    }

    fn add_buf_statements_to_frame(&mut self, buf: &Vec<Statement>) {
        // println!("current range {:?}", self.current_range);
        // println!("current buf {:?}", buf);
        let mut buf_statements: Vec<Statement> = vec![];
        for fps in self.current_range.clone() {
            // println!("fps {}  (added + 1)", fps + 1);
            for statement in buf {
                buf_statements.push(statement.clone());
            }
            self.frames
                .entry(fps + 1)
                .and_modify(|x| x.extend(buf_statements.clone()))
                .or_insert(buf_statements.clone());
            buf_statements.clear();
        }
    }

    fn get_fps_duration_from_statement(&self, environment: &mut Environment, fps_statement: &Statement) -> Result<usize> {
        match fps_statement {
            Statement::Fps(next_fps) => Ok(self.get_fps_duration_from_token(&next_fps)),
            Statement::FpsEnd(_) => Ok(1),
            Statement::For { expr: range, for_block: _ } => {
                let range = range.eval(environment)?;
                match range {
                    AstLiteralValue::Range((start, end)) => Ok(end - start - 1),
                    AstLiteralValue::RangeEqual((start, end)) => Ok(end - start),
                    _ => panic!(),
                }
            }
            _ => panic!("cannot retrieve fps duration from this statement {:?}", fps_statement),
        }
    }

    pub fn allocate_statements_to_frame(&mut self, environment: &mut Environment, statements: Vec<Statement>) -> Result<()> {
        let mut buf_fps_statements: Vec<Statement> = vec![];

        for statement in statements {
            match statement {
                Statement::Fps(_) => {
                    self.add_buf_statements_to_frame(&buf_fps_statements);
                    let next_fps: usize = self.get_fps_duration_from_statement(environment, &statement)?;
                    self.current_range = self.current_range.end..self.current_range.end + next_fps;
                    buf_fps_statements.clear();
                }
                Statement::FpsEnd(_) => {
                    self.add_buf_statements_to_frame(&buf_fps_statements);
                    self.current_range = self.current_range.end..self.current_range.end;
                    buf_fps_statements.clear();
                }
                Statement::Comment(_) => {
                    // ignore
                }
                Statement::For { expr: _, ref for_block } => {
                    // clear buf first (if statements before the for block?)
                    self.add_buf_statements_to_frame(&buf_fps_statements);
                    buf_fps_statements.clear();
                    // close test

                    let current_range = self.current_range.clone();

                    let fps_duration = self.get_fps_duration_from_statement(environment, &statement)?;
                    buf_fps_statements.extend(for_block.clone());
                    if self.current_range.end == 1 { // At frame 0 we can't subtract by 1 at the end
                        self.current_range = self.current_range.start..self.current_range.end + (self.current_range.end * fps_duration);
                    } else {
                        self.current_range = self.current_range.start..self.current_range.end + (self.current_range.end * fps_duration) - 1;
                    }
                    // println!("current_range for {:?}", self.current_range);
                    self.add_buf_statements_to_frame(&buf_fps_statements);
                    buf_fps_statements.clear();

                    self.current_range = current_range;
                }
                _ => buf_fps_statements.push(statement.clone()),
            }
        }

        // println!("> frames: {:?}", self.frames);

        Ok(())
    }
}
