use std::collections::BTreeMap;
use anyhow::Result;
use thiserror::Error;

use super::literal::LiteralValue;


#[derive(Error, Debug)]
enum AstError {
    #[error("Cannot declare variable '{0}' as it is already defined")]
    Declaration(String),
    #[error("Variable '{0}' is not yet declared")]
    NotDeclared(String),
}



pub struct Environment {
    variables: BTreeMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Result<LiteralValue> {
        match self.variables.get(&name) {
            Some(val) => Ok(val.clone()),
            None => return Err(AstError::NotDeclared(name).into())
        }
    }

    pub fn declare(&mut self, name: String, value: LiteralValue) -> Result<()>{
        if self.variables.contains_key(&name) {
            return Err(AstError::Declaration(name).into());
        }

        self.variables.insert(name.clone(), value);

        Ok(())
    }
}
