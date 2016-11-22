use z3_sys::*;
use Context;
use Tactic;
use Goal;
use Z3_MUTEX;
use std::ffi::CString;

impl <'ctx> Tactic<'ctx> {

    pub fn from_name(ctx: &'ctx Context, name: &str) -> Tactic<'ctx> {
        Tactic {
            ctx: ctx,
            z3_tactic: unsafe {
                let z3_name = CString::new(name).unwrap();
                let guard = Z3_MUTEX.lock().unwrap();
                let t = Z3_mk_tactic(ctx.z3_ctx, z3_name.as_ptr());
                Z3_tactic_inc_ref(ctx.z3_ctx, t);
                t
            }
        }
    }

    pub fn apply(&self, goal: Goal<'ctx>) -> Vec<Goal<'ctx>> {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            let z3_result = Z3_tactic_apply(self.ctx.z3_ctx, self.z3_tactic, goal.z3_goal);
            Z3_apply_result_inc_ref(self.ctx.z3_ctx, z3_result);
            let result_count = Z3_apply_result_get_num_subgoals(self.ctx.z3_ctx, z3_result);
            let mut result = vec!();
            for i in 0..result_count {
                let goal_result = Z3_apply_result_get_subgoal(self.ctx.z3_ctx, z3_result, i);
                result.push(Goal {
                    ctx: self.ctx,
                    z3_goal: goal_result
                });
                Z3_goal_inc_ref(self.ctx.z3_ctx, goal_result);
            }
            Z3_apply_result_dec_ref(self.ctx.z3_ctx, z3_result);
            result
        }
    }

}

impl<'ctx> Drop for Tactic<'ctx> {
    fn drop(&mut self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_tactic_dec_ref(self.ctx.z3_ctx, self.z3_tactic);
        }
    }
}