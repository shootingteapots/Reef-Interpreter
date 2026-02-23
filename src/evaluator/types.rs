use reef_syntax::common::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtRes};

/// Types utilised in the evaluator. Every statement evaluates to None, while
/// expressions can be that of a Number, a String, or a Boolean.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeType {
    Number(f64),
    String(String),
    Boolean(Boolean),
    None,
}

#[derive(Debug, Clone)]
pub struct VariableError(pub String);

/// Structure used for storing variables in a program. Can have an optional
/// parent for nested scopes.
#[derive(Debug)]
pub struct Scope<'a> {
    variables: HashMap<String, RuntimeType>,
    parent: Option<&'a mut Scope<'a>>,
}

impl<'a> Display for Scope<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        write!(
            f,
            "Scope<parent: {:?}, variables: {:?}>",
            self.parent, self.variables
        )
    }
}

impl<'a> Scope<'a> {
    pub fn new(parent: Option<&'a mut Scope<'a>>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: parent,
        }
    }

    /// Attempts to get a variable from self.variables. If it doesn't exist, it
    /// tries to check the parent scope for the same variable. Panics if it can't
    /// find the variable at all.
    pub fn get_variable(&self, name: &str) -> Result<RuntimeType, VariableError> {
        let v = self.variables.get(name);

        match v {
            Some(v) => Ok(v.clone()),
            None => match &self.parent {
                Some(parent) => match parent.get_variable(name) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                None => Err(VariableError(format!("No variable called {} exists", name))),
            },
        }
    }

    /// Sets the variable <name> to <value> in self.variables.
    pub fn set_variable(
        &mut self,
        name: &str,
        value: RuntimeType,
    ) -> Result<RuntimeType, VariableError> {
        if self.variables.contains_key(name) {
            Err(VariableError(format!(
                "Variable named {name} already exists. Did you mean to reassign it?"
            )))
        } else {
            self.variables.insert(name.to_string(), value);
            Ok(RuntimeType::None)
        }
    }

    /// Updates the value of variable <name> to <value>. Panics if the variable
    /// doesn't exist
    pub fn reassign_variable(
        &mut self,
        name: &str,
        value: RuntimeType,
    ) -> Result<RuntimeType, VariableError> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(RuntimeType::None)
        } else {
            Err(VariableError(format!(
                "Attempt to reassign variable \"{name}\" which doesn't exist."
            )))
        }
    }
}

impl Display for VariableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        write!(f, "{}", self.0)
    }
}

impl Display for RuntimeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        match self {
            Self::None => write!(f, "None")?,
            Self::Number(number) => write!(f, "{}", number)?,
            Self::String(string) => write!(f, "{}", string)?,
            Self::Boolean(boolean) => write!(
                f,
                "{}",
                match boolean {
                    Boolean::True => "true",
                    Boolean::False => "false",
                }
            )?,
        }

        Ok(())
    }
}
