use z3_sys::*;
use Context;
use Goal;
use Ast;
use Z3_MUTEX;

impl <'ctx> Goal<'ctx> {

    pub fn new(ctx: &'ctx Context, models: bool, unsat_cores: bool, proofs: bool) -> Goal {
        Goal {
            ctx: ctx,
            z3_goal: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let g = Z3_mk_goal(ctx.z3_ctx,
                            if models { 1 } else { 0 },
                            if unsat_cores { 1 } else { 0 },
                            if proofs { 1 } else { 0 }
                );
                Z3_goal_inc_ref(ctx.z3_ctx, g);
                g
            }
        }
    }

    pub fn assert(&self, ast: &'ctx Ast) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_goal_assert(self.ctx.z3_ctx, self.z3_goal, ast.z3_ast);
        }
    }

    pub fn formulas(&self) -> Vec<Ast<'ctx>> {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let size = Z3_goal_num_exprs(self.ctx.z3_ctx, self.z3_goal);
            let mut result = vec!();
            for i in 0..size {
                let ast = Z3_goal_formula(self.ctx.z3_ctx, self.z3_goal, i);
                Z3_inc_ref(self.ctx.z3_ctx, ast);
                result.push(Ast {
                    ctx: self.ctx,
                    z3_ast: ast
                })
            }
            result
        }
    }

    pub fn reset(&self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_goal_reset(self.ctx.z3_ctx, self.z3_goal);
        }
    }

}


impl<'ctx> Drop for Goal<'ctx> {
    fn drop(&mut self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_goal_dec_ref(self.ctx.z3_ctx, self.z3_goal);
        }
    }
}