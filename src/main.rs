extern crate z3;
use self::z3::{Context, Config, Ast};


fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = ctx.named_int_const("x");
    let y = ctx.named_int_const("y");
    let zero = ctx.from_i64(0);
    let two = ctx.from_i64(2);
    let seven = ctx.from_i64(7);

    let formula = (x.gt(&y)).and(&[&y.gt(&zero), &y.add(&[&seven])._eq(&two)]);

    let expected_string = "(declare-const x Int)\n(assert (= (+ 1 2) x))";
    let real_string = formula.as_smtlib2_string();
    //assert_eq!(expected_string, real_string);

    let x_sym = ctx.str_sym("x");
    let x_sort = ctx.int_sort();
    let y_sym = ctx.str_sym("y");
    let y_sort = ctx.int_sort();

    let symbols = vec!((&x_sym, &x_sort), (&y_sym, &y_sort));

    println!("Str: ${:?}", real_string);
    println!("Formula: ${:?}", formula);
    let r = Ast::from_smtlib2_string(&ctx, expected_string);
    println!("Reconstruction: {:?}", r);
}