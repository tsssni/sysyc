use crate::ir::{Error, Result};
use crate::ir::function::FunctionInfo;
use std::vec::Vec;
use std::collections::HashMap;
use koopa::ir::{Value, Function};

pub struct Context<'ast> {
    value_blocks: Vec<HashMap<&'ast str, Value>>,
    functions: HashMap<&'ast str, Function>,
    pub active_fcuntion: Option<FunctionInfo>,
}

impl<'ast> Context<'ast> {
    pub fn new() -> Self {
        Self {
            value_blocks: vec![HashMap::new()],
            functions: HashMap::new(),
            active_fcuntion: None,
        }
    }

    pub fn active_block(&self) -> &HashMap<&'ast str, Value> {
        self.value_blocks.last().unwrap()
    }

    pub fn active_block_mut(&mut self) -> &mut HashMap<&'ast str, Value> {
        self.value_blocks.last_mut().unwrap()
    }

    pub fn active_fcuntion(&self) -> &FunctionInfo {
        self.active_fcuntion.as_ref().unwrap()
    }

    pub fn active_fcuntion_mut(&mut self) -> &mut FunctionInfo {
        self.active_fcuntion.as_mut().unwrap()
    }

    pub fn insert_value(&mut self, id: &'ast str, value: Value) -> Result<()> {
        let cur = self.active_block_mut();
        if cur.contains_key(id) {
            Err(Error::DuplicateDefinition)
        } else {
            cur.insert(id, value);
            Ok(())
        }
    }

    pub fn get_value(&self, id: &str) -> Result<Value> {
        let mut block_i = self.value_blocks.len() - 1;
        while block_i >= 0 {
            match self.value_blocks[block_i].get(id) {
                Some(value) => return Ok(value.clone()),
                None => block_i -= 1,
            }
        }
        Err(Error::SymbolNotFound)
    }

    pub fn insert_function(&mut self, id: &'ast str, func: Function) -> Result<()> {
        if self.functions.contains_key(id) {
            Err(Error::DuplicateDefinition)
        } else {
            self.functions.insert(id, func);
            Ok(())
        }
    }

    pub fn get_function(&self, id: &str) -> Result<Function> {
        self.functions.get(id).cloned().ok_or(Error::SymbolNotFound)
    }

    pub fn push(&mut self) {
        self.value_blocks.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.value_blocks.pop();
    }
}
