use anyhow::Result;
use std::{collections::BTreeMap, ops::Range};

use super::statement::Statement;
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
            self.frames.entry(fps + 1).or_insert(buf_statements.clone());
            buf_statements.clear();
        }
    }

    fn get_fps_duration_from_statement(&self, fps_statement: &Statement) -> usize {
        match fps_statement {
            Statement::Fps(next_fps) => self.get_fps_duration_from_token(&next_fps),
            Statement::FpsEnd(_) => 1,
            _ => 0,
        }
    }

    pub fn allocate_statements_to_frame(&mut self, statements: Vec<Statement>) -> Result<()> {
        let mut buf_fps_statements: Vec<Statement> = vec![];

        for statement in statements {
            if matches!(statement, Statement::Fps(_)) {
                self.add_buf_statements_to_frame(&buf_fps_statements);
                let next_fps: usize = self.get_fps_duration_from_statement(&statement);
                self.current_range = self.current_range.end..self.current_range.end + next_fps;
                buf_fps_statements.clear();
            } else if matches!(statement, Statement::FpsEnd(_)) {
                self.add_buf_statements_to_frame(&buf_fps_statements);
                self.current_range = self.current_range.end..self.current_range.end;
                buf_fps_statements.clear();
            } else if matches!(statement, Statement::Comment(_)) {
                // ignore
            } else {
                // println!("\t> inserting statement {:?}", statement);
                buf_fps_statements.push(statement.clone());
            }
        }

        // println!("> frames: {:?}", self.frames);

        Ok(())
    }
}
