#![allow(dead_code)]
#[cfg(feature = "without_std")]
use hashbrown::HashMap;
#[cfg(not(feature = "without_std"))]
use std::collections::HashMap;

// TODO:L find a way to make the arguments generic;
#[derive(Clone, Debug)]
pub enum ArgsBox {
    Float(f32),
    Int(i32),
    Str(String),
    Usize(usize),
}

// TODO: make this optional
pub struct Args {
    args: HashMap<String, ArgsBox>,
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Self {
        Args {
            args: HashMap::new(),
        }
    }

    pub fn fget(&self, key: &str) -> Option<f32> {
        let val = self.args.get(key)?;
        match val {
            ArgsBox::Float(s) => Some(*s),
            _ => None,
        }
    }

    pub fn iget(&self, key: &str) -> Option<i32> {
        let val = self.args.get(key)?;
        match val {
            ArgsBox::Int(s) => Some(*s),
            _ => None,
        }
    }

    pub fn sget(&self, key: &str) -> Option<String> {
        let val = self.args.get(key)?;
        match val {
            ArgsBox::Str(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn uget(&self, key: &str) -> Option<usize> {
        let val = self.args.get(key)?;
        match val {
            ArgsBox::Usize(s) => Some(*s),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<ArgsBox> {
        let val = self.args.get(key)?;
        Some(val.clone())
    }

    pub fn fset(&mut self, key: &str, value: f32) -> &mut Args {
        self.args.insert(key.to_string(), ArgsBox::Float(value));
        self
    }

    pub fn iset(&mut self, key: &str, value: i32) -> &mut Args {
        self.args.insert(key.to_string(), ArgsBox::Int(value));
        self
    }

    pub fn uset(&mut self, key: &str, value: usize) -> &mut Args {
        self.args.insert(key.to_string(), ArgsBox::Usize(value));
        self
    }

    pub fn sset(&mut self, key: &str, value: &str) -> &mut Args {
        self.args
            .insert(key.to_string(), ArgsBox::Str(value.to_string()));
        self
    }

    pub fn set(&mut self, key: &str, value: ArgsBox) -> &mut Args {
        self.args.insert(key.to_string(), value);
        self
    }
}
