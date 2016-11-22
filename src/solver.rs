use z3_sys::*;
use Context;
use Solver;
use Model;
use Ast;
use Tactic;
use Symbol;
use Z3_MUTEX;

impl<'ctx> Solver<'ctx> {
    pub fn new(ctx: &Context) -> Solver {
        Solver {
            ctx: ctx,
            z3_slv: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let s = Z3_mk_solver(ctx.z3_ctx);
                Z3_solver_inc_ref(ctx.z3_ctx, s);
                s
            }
        }
    }

    pub fn new_with_tactic(ctx: &'ctx Context, tactic: &'ctx Tactic) -> Solver<'ctx> {
        Solver {
            ctx: ctx,
            z3_slv: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let s = Z3_mk_solver_from_tactic(ctx.z3_ctx, tactic.z3_tactic);
                Z3_solver_inc_ref(ctx.z3_ctx, s);
                s
            }
        }
    }

    pub fn new_with_logic(ctx: &'ctx Context, logic: &str) -> Solver<'ctx> {
        Solver {
            ctx: ctx,
            z3_slv: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                let logic_symbol = Symbol::from_string(ctx, logic);
                let s = Z3_mk_solver_for_logic(ctx.z3_ctx, logic_symbol.z3_sym);
                Z3_solver_inc_ref(ctx.z3_ctx, s);
                s
            }
        }
    }

    pub fn assert(&self, ast: &Ast<'ctx>) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_solver_assert(self.ctx.z3_ctx,
                             self.z3_slv,
                             ast.z3_ast);
        }
    }

    pub fn check(&self) -> bool {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_solver_check(self.ctx.z3_ctx,
                            self.z3_slv) == Z3_TRUE
        }
    }

    pub fn reset(&self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_solver_reset(self.ctx.z3_ctx, self.z3_slv);
        }
    }

    pub fn get_model(&self) -> Model<'ctx> {
        Model::of_solver(self)
    }
}


impl<'ctx> Drop for Solver<'ctx> {
    fn drop(&mut self) {
        unsafe {
            let guard = Z3_MUTEX.lock().unwrap();
            Z3_solver_dec_ref(self.ctx.z3_ctx, self.z3_slv);
        }
    }
}
