use anyhow::Result;
use std::{collections::BTreeMap, rc::Rc};
use thiserror::Error;

use super::literal::LiteralValue;

#[derive(Error, Debug)]
enum AstError {
    #[error("Cannot declare variable '{0}' as it is already defined")]
    AlreadyDeclared(String),
    #[error("Variable '{0}' is not yet declared in current scope")]
    NotDeclared(String),
}

// #[derive(Debug)]
// pub struct Variable {
//     // pub fps: Fps,
//     pub value: LiteralValue,
// }

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Rc<Environment>>,
    variables: BTreeMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: BTreeMap::new(),
        }
    }

    fn resolve(&mut self, name: String) -> Result<&mut Self> {
        if self.variables.contains_key(&name) {
            return Ok(self);
        }

        if let Some(parent) = &mut self.parent {
            return Rc::get_mut(parent).expect("Could not get mutable reference to environment").resolve(name);
        } else {
            return Err(AstError::NotDeclared(name).into());
        }
    }

    pub fn get(&mut self, name: String) -> Result<LiteralValue> {
        let env = self.resolve(name.clone())?;

        match env.variables.get(&name) {
            Some(val) => Ok(val.clone()),
            None => return Err(AstError::NotDeclared(name).into()),
        }
    }

    pub fn declare(&mut self, name: String, value: LiteralValue) -> Result<()> {
        if self.variables.contains_key(&name) {
            return Err(AstError::AlreadyDeclared(name).into());
        }

        self.variables.insert(name.clone(), value);

        Ok(())
    }

    pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<()> {
        let env = self.resolve(name.clone())?;

        env.variables.entry(name).and_modify(|v| *v = value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declare() {
        let mut env = Environment::new();
        env.declare("a".to_string(), LiteralValue::Boolean(false)).unwrap();

        let expected: BTreeMap<String, LiteralValue> = BTreeMap::from([("a".to_owned(), LiteralValue::Boolean(false))]);

        assert_eq!(env.variables, expected);
    }

    #[test]
    fn assign() {
        let mut env = Environment::new();
        env.declare("a".to_string(), LiteralValue::Boolean(false)).unwrap();
        env.assign("a".to_string(), LiteralValue::Boolean(true)).unwrap();

        let expected: BTreeMap<String, LiteralValue> = BTreeMap::from([("a".to_owned(), LiteralValue::Boolean(true))]);

        assert_eq!(env.variables, expected);
    }

    #[test]
    fn declare_different_env() {
        let mut parent_env = Environment::new();
        parent_env.declare("a".to_string(), LiteralValue::Boolean(false)).unwrap();
        
        let mut child_env = Environment {
            parent: Some(Rc::new(parent_env.clone())),
            variables: BTreeMap::new(),
        };
        child_env.declare("a".to_string(), LiteralValue::Boolean(true)).unwrap();

        let expected_parent: BTreeMap<String, LiteralValue> = BTreeMap::from([("a".to_owned(), LiteralValue::Boolean(false))]);
        let expected_child: BTreeMap<String, LiteralValue> = BTreeMap::from([("a".to_owned(), LiteralValue::Boolean(true))]);

        assert_eq!(parent_env.variables, expected_parent);
        assert_eq!(child_env.variables, expected_child);
    }
}
