#[macro_use]
extern crate log;
extern crate env_logger;

extern crate z3;
use z3::*;

#[test]
fn test_config() {
    let _ = env_logger::init();
    let mut c = Config::new();
    c.set_proof_generation(true);
}

#[test]
fn test_context() {
    let _ = env_logger::init();
    let mut cfg = Config::new();
    cfg.set_proof_generation(true);
    let _ = Context::new(&cfg);
}

#[test]
fn test_sorts_and_symbols() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let _ = ctx.named_int_const("x");
    let _ = ctx.named_int_const("y");
}

#[test]
fn test_solving() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&y));
    assert!(solver.check());
}

#[test]
fn test_solving_for_model() {
    let _ = env_logger::init();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let zero = ctx.from_i64(0);
    let two = ctx.from_i64(2);
    let seven = ctx.from_i64(7);

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&y));
    solver.assert(&y.gt(&zero));
    solver.assert(&y.rem(&seven)._eq(&two));
    solver.assert(&x.add(&[&two]).gt(&seven));
    assert!(solver.check());

    let model = solver.get_model();
    let xv = model.eval(&x).unwrap().as_i64().unwrap();
    let yv = model.eval(&y).unwrap().as_i64().unwrap();
    info!("x: {}", xv);
    info!("y: {}", yv);
    assert!(xv > yv);
    assert!(yv % 7 == 2);
    assert!(xv + 2 > 7);
}

#[test]
fn test_ast_to_string() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let zero = ctx.from_i64(0);
    let two = ctx.from_i64(2);
    let seven = ctx.from_i64(7);

    let formula = (x.gt(&y)).and(&[&y.gt(&zero), &y.add(&[&seven])._eq(&two)]);

    let expected_string = "(and (> x y) (> y 0) (= (+ y 7) 2))";
    let real_string = formula.as_smtlib2_string();

    assert_eq!(expected_string, real_string);

    /*
    let x_sym = ctx.str_sym("x");
    let x_sort = ctx.int_sort();
    let y_sym = ctx.str_sym("y");
    let y_sort = ctx.int_sort();

    let symbols = vec!((&x_sym, &x_sort), (&y_sym, &y_sort));

    let reconstructed = Ast::from_smtlib2_string(&ctx, &real_string, &symbols);

    assert_eq!(formula, reconstructed)
    */
}

#[test]
fn test_string_to_ast() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let zero = ctx.from_i64(0);
    let two = ctx.from_i64(2);
    let seven = ctx.from_i64(7);

    let expected_formula = (x.gt(&y)).and(&[&y.gt(&zero), &y.add(&[&seven])._eq(&two)]);

    let formula_string = "(declare-const x Int)(declare-const y Int)(assert (and (> x y) (> y 0) (= (+ y 7) 2)))";
    let real_formula = Ast::from_smtlib2_string(&ctx, formula_string);

    assert_eq!(expected_formula, real_formula);
}