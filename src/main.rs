extern crate z3;
use self::z3::{Context, Config, Ast, Tactic, Goal};
extern crate z3_sys;
use self::z3_sys::*;
use std::ffi::CString;

fn main() {
    let z3 = Context::new(&Config::new());
    let str = "(declare-const k!0 Real)
(assert (let ((a!1 (or false
               (> (+ (/ 2813.0 6250.0) (* k!0 0.0)) 0.0)
               (> (+ (/ 428273.0 1000000.0) (* k!0 0.0)) 0.0))))
  (and a!1 true true (> k!0 0.0) (< k!0 1.0))))";
    let f = Ast::from_smtlib2_string(&z3, str);
    println!("Ast: {:?}", f);
    let name = CString::new("ctx-solver-simplify").unwrap();
    let t = Z3_mk_tactic(z3.z3_ctx, name.as_ptr());
    //let tactic = z3.z3_ctx//Tactic::from_name(&z3, "ctx-solver-simplify");
    let goal = Goal::new(&z3, false, false, false);
    goal.assert(&f);
    let results = t.apply(goal);
    if results.len() != 1 {
        panic!("Unexpected number of goals after optimisation")
    } else {
        let formulas = results[0].formulas();
        println!("Simplified: {:?}", formulas);
    }
}