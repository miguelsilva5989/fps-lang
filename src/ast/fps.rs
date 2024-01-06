use anyhow::Result;
use std::ops::Range;

use super::statement::Statement;
use crate::lexer::{LiteralValue, Token};

#[derive(Debug)]
pub struct Fps {
    pub frames: Vec<(Range<usize>, Vec<Statement>)>,
    current_fps_duration: usize,
    current_range: Range<usize>,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            frames: vec![(0..1, vec![])],
            current_fps_duration: 1,
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

    // fn initialize_next_frame(&mut self, fps_count: usize) {
    //     self.frames
    //         .insert(self.current_range.end+1..(self.current_range.end + fps_count + 1), vec![]);
    // }
    fn initialize_next_frame(&mut self, range: Range<usize>) {
        self.frames.push((range, vec![]));
    }

    fn add_buf_statements_to_frame(&mut self, buf: Vec<Statement>) {
        // println!("current range {:?}", self.current_range);

        let mut buf_statemetns: Vec<Statement> = vec![];
        for _ in self.current_range.clone() {
            for statement in buf.clone() {
                buf_statemetns.push(statement.clone());
            }
        }

        let idx = self.frames.iter().position(|(range, _)| range == &self.current_range);
        if let Some(idx) = idx {
            self.frames[idx] = (self.current_range.clone(), buf_statemetns);
        }
    }

    fn get_fps_duration_from_statement(&self, fps_statement: &Statement) -> usize {
        match fps_statement {
            Statement::Fps(next_fps) => self.get_fps_duration_from_token(&next_fps),
            Statement::FpsEnd(_) => 1,
            _ => 0,
        }
    }

    fn eval_frame_statement(&mut self, fps_statement: Statement, buf: Vec<Statement>) {
        // println!("> current buf: {:?}", buf);
        // self.current_range = self.current_fps..self.current_fps_duration;

        match fps_statement {
            Statement::Fps(next_fps) => {
                let fps_duration = self.get_fps_duration_from_token(&next_fps);
                // self.initialize_next_frame(fps_duration);

                self.current_fps_duration = fps_duration;
            }
            Statement::FpsEnd(_) => {
                // let fps_duration = 1;
                // self.initialize_next_frame(fps_duration);

                // self.current_fps_duration = 0;
            }
            _ => {}
        };

        self.add_buf_statements_to_frame(buf);

        // println!("FPS: {:?}", self.frames);
    }

    pub fn allocate_statements_to_frame(&mut self, statements: Vec<Statement>) -> Result<()> {
        let mut buf_fps_statements: Vec<Statement> = vec![];
        for statement in statements {
            // println!("> Checking statement {:?}", statement);
            if matches!(statement, Statement::Fps(_)) {
                let next_fps: usize = self.get_fps_duration_from_statement(&statement);
                self.eval_frame_statement(statement, buf_fps_statements.clone());
                buf_fps_statements = vec![];

                // self.current_fps = next_fps;

                // self.current_fps = self.current_fps + fps_duration;
                // println!("\t> after dump {:?}", self);

                self.current_range = self.current_range.end..self.current_range.end + next_fps;
                self.initialize_next_frame(self.current_range.clone());

                // println!("!!set range to {:?}", self.current_range);
            } else if matches!(statement, Statement::Comment(_)) {
                // ignore
            } else if matches!(statement, Statement::FpsEnd(_)) {
                // println!("FPS END");
                self.eval_frame_statement(statement, buf_fps_statements.clone());
                buf_fps_statements = vec![];
                // println!("\tafter end {:?}", self);

                self.current_range = self.current_range.end..self.current_range.end;
            } else {
                // println!("\t> inserting statement {:?}", statement);
                buf_fps_statements.push(statement.clone());
            }
        }

        // println!("buf {:?}", buf_fps_statements);
        // println!("{:?}", self);

        Ok(())
    }
}
