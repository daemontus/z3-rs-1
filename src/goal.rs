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