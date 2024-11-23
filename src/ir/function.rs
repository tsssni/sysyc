use koopa::ir::Value;
use koopa::ir::{builder::LocalBuilder, builder_traits::*};
use koopa::ir::{BasicBlock, Function, Program, Type};

pub struct FunctionInfo {
    function: Function,
    entry: BasicBlock,
    end: BasicBlock,
    active: BasicBlock,
    return_value: Option<Value>,
}

impl FunctionInfo {
    pub fn new(function: Function, entry: BasicBlock, end: BasicBlock, return_value: Option<Value>) -> Self {
        Self {
            function,
            entry,
            end,
            active: entry,
            return_value,
        }
    }

    pub fn function(&self) -> Function {
        self.function
    }

    pub fn end(&self) -> BasicBlock {
        self.end
    }

    pub fn return_value(&self) -> Option<Value> {
        self.return_value
    }

    pub fn create_basic_block(&self, program: &mut Program, name: Option<&str>) -> BasicBlock {
        program
        .func_mut(self.function)
        .dfg_mut()
        .new_bb()
        .basic_block(name.map(|s| s.into()))
    }

    pub fn create_value<'p>(&self, program: &'p mut Program) -> LocalBuilder<'p> {
        program.func_mut(self.function).dfg_mut().new_value()
    }

    pub fn push_basic_block(&mut self, program: &mut Program, bb: BasicBlock) {
        program
        .func_mut(self.function)
        .layout_mut()
        .bbs_mut()
        .push_key_back(bb)
        .unwrap();
        self.active = bb;
    }

    pub fn insert_instruction(&self, program: &mut Program, bb: BasicBlock, inst: Value) {
        program
        .func_mut(self.function)
        .layout_mut()
        .bb_mut(bb)
        .insts_mut()
        .push_key_back(inst)
        .unwrap();
    }

    pub fn push_instruction(&self, program: &mut Program, inst: Value) {
        self.insert_instruction(program, self.active, inst);
    }

    pub fn allocate(&self, program: &mut Program, ty: Type, name: Option<&str>) -> Value {
        let alloc = self.create_value(program).alloc(ty);
        if let Some(name) = name {
            program
            .func_mut(self.function)
            .dfg_mut()
            .set_value_name(alloc, Some(format!("@{}", name)));
        }
        self.insert_instruction(program, self.entry, alloc);
        alloc
    }
}
