use crate::ir::GenerateIR;
use crate::ir::{Result, Error};
use crate::ir::context::Context;
use crate::ast::*;
use koopa::ir::builder::LocalInstBuilder;
use koopa::ir::Program;

impl<'ast> GenerateIR<'ast> for Block {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        context.push();
        for item in &self.items {
            item.generate(program, context)?;
        }
        context.pop();
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for BlockItem {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            BlockItem::Stmt(stmt) => stmt.generate(program, context),
            BlockItem::Decl(decl) => decl.generate(program, context),
        }
    }
}

impl<'ast> GenerateIR<'ast> for Stmt {
    type Out = ();

    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        match self {
            Stmt::Assign(assign) => assign.generate(program, context),
            Stmt::Sblock(sblock) => sblock.generate(program, context),
            Stmt::If(sif) => sif.generate(program, context),
            Stmt::While(swhile) => swhile.generate(program, context),
            Stmt::Return(ret) => ret.generate(program, context),
        }
    }
}

impl<'ast> GenerateIR<'ast> for Assign {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let exp = self.exp.generate(program, context)?;
        let lval = context.get_value(&self.lval.id)?;
        let active_func = context.active_function();
        let store = active_func.create_value(program).store(exp, lval);
        active_func.push_instruction(program, store);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Sblock {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        self.block.generate(program, context)
    }
}

impl<'ast> GenerateIR<'ast> for If {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let cond = self.cond.generate(program, context)?;
        let active_func = context.active_function_mut();

        let then_bb = active_func.create_basic_block(program, "%then".into());
        let else_bb = active_func.create_basic_block(program, "%else".into());
        let next_bb = active_func.create_basic_block(program, "%body".into());
        let br = active_func.create_value(program).branch(cond, then_bb, else_bb);
        active_func.push_instruction(program, br);

        active_func.push_basic_block(program, then_bb);
        self.then_block.generate(program, context)?;
        let active_func = context.active_function_mut();
        let then_jump = active_func.create_value(program).jump(next_bb);
        active_func.push_instruction(program, then_jump);

        active_func.push_basic_block(program, else_bb);
        if let Some(else_block) = &self.else_block {
            else_block.generate(program, context)?;
        }
        let active_func = context.active_function_mut();
        let else_jump = active_func.create_value(program).jump(next_bb);
        active_func.push_instruction(program, else_jump);
        
        active_func.push_basic_block(program, next_bb);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for While {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        let active_func = context.active_function_mut();
        let entry_bb = active_func.create_basic_block(program, "%while_entry".into());
        active_func.push_basic_block(program, entry_bb);
        let cond = self.cond.generate(program, context)?;

        let active_func = context.active_function_mut();
        let body_bb = active_func.create_basic_block(program, "%while_body".into());
        let next_bb = active_func.create_basic_block(program, "%body".into());
        let br = active_func.create_value(program).branch(cond, body_bb, next_bb);
        active_func.push_instruction(program, br);

        active_func.push_basic_block(program, body_bb);
        self.block.generate(program, context)?;
        let active_func = context.active_function_mut();
        let jump = active_func.create_value(program).jump(entry_bb);
        active_func.push_instruction(program, jump);

        active_func.push_basic_block(program, next_bb);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Return {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context: &mut Context<'ast>) -> Result<Self::Out> {
        if let None = context.active_function().return_value() {
            return Err(Error::ReturnInVoidFunction);
        }
        let exp = self.exp.generate(program, context)?;

        let active_func = context.active_function_mut();
        let jump = active_func.create_value(program).jump(active_func.end());
        active_func.finish_allocate(program);
        active_func.push_instruction(program, jump);
        active_func.push_basic_block(program, active_func.end());

        let value = active_func.return_value().map(|alloc| {
            let store = active_func.create_value(program).store(exp, alloc);
            active_func.push_instruction(program, store);

            let value = active_func.create_value(program).load(alloc);
            active_func.push_instruction(program, value);
            value
        });
        let ret = active_func.create_value(program).ret(value);
        active_func.push_instruction(program, ret);
        Ok(())
    }
}
