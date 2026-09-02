use crate::Realm;
use crate::Value;

fn eval(src: &str) -> Value {
    let mut realm = Realm::new();
    match realm.eval(src) {
        Ok(v) => v,
        Err(e) => panic!("eval error for {src:?}: {e}"),
    }
}

fn eval_err(src: &str) -> String {
    let mut realm = Realm::new();
    match realm.eval(src) {
        Ok(v) => format!("OK: {v}"),
        Err(e) => e.to_string(),
    }
}

// ─── Literals ───

#[test]
fn number_literal() {
    assert_eq!(eval("42").to_number(), 42.0);
    assert_eq!(eval("3.15").to_number(), 3.15);
    assert_eq!(eval("0").to_number(), 0.0);
    assert_eq!(eval(".5").to_number(), 0.5);
}

#[test]
fn string_literal() {
    assert_eq!(eval("'hello'").to_string_value(), "hello");
    assert_eq!(eval("\"world\"").to_string_value(), "world");
    assert_eq!(eval("'line\\non'").to_string_value(), "line\non");
    assert_eq!(eval("'tab\\there'").to_string_value(), "tab\there");
}

#[test]
fn bool_literal() {
    assert_eq!(eval("true"), Value::Bool(true));
    assert_eq!(eval("false"), Value::Bool(false));
}

#[test]
fn null_and_undefined() {
    assert_eq!(eval("null"), Value::Null);
    assert_eq!(eval("undefined"), Value::Undefined);
}

// ─── Arithmetic ───

#[test]
fn addition() {
    assert_eq!(eval("1 + 2").to_number(), 3.0);
    assert_eq!(eval("0 + 0").to_number(), 0.0);
    assert_eq!(eval("-1 + 1").to_number(), 0.0);
}

#[test]
fn subtraction() {
    assert_eq!(eval("5 - 3").to_number(), 2.0);
    assert_eq!(eval("3 - 5").to_number(), -2.0);
}

#[test]
fn multiplication() {
    assert_eq!(eval("3 * 4").to_number(), 12.0);
    assert_eq!(eval("0 * 100").to_number(), 0.0);
}

#[test]
fn division() {
    assert_eq!(eval("10 / 2").to_number(), 5.0);
    assert_eq!(eval("7 / 2").to_number(), 3.5);
}

#[test]
fn modulo() {
    assert_eq!(eval("10 % 3").to_number(), 1.0);
    assert_eq!(eval("7 % 2").to_number(), 1.0);
}

#[test]
fn negation() {
    assert_eq!(eval("-5").to_number(), -5.0);
    assert_eq!(eval("-(-3)").to_number(), 3.0);
}

#[test]
fn precedence_mul_before_add() {
    assert_eq!(eval("2 + 3 * 4").to_number(), 14.0);
    assert_eq!(eval("2 * 3 + 4").to_number(), 10.0);
}

#[test]
fn precedence_parens() {
    assert_eq!(eval("(2 + 3) * 4").to_number(), 20.0);
    assert_eq!(eval("((1 + 2) * (3 + 4))").to_number(), 21.0);
}

#[test]
fn nested_arithmetic() {
    assert_eq!(eval("1 + 2 + 3 + 4").to_number(), 10.0);
    assert_eq!(eval("10 - 3 - 2").to_number(), 5.0);
    assert_eq!(eval("2 * 3 * 4").to_number(), 24.0);
}

// ─── Comparisons ───

#[test]
fn eq_numbers() {
    assert_eq!(eval("1 == 1"), Value::Bool(true));
    assert_eq!(eval("1 == 2"), Value::Bool(false));
}

#[test]
fn strict_eq() {
    assert_eq!(eval("1 === 1"), Value::Bool(true));
    assert_eq!(eval("1 === 1.0"), Value::Bool(true));
    assert_eq!(eval("1 === '1'"), Value::Bool(false));
}

#[test]
fn ne_numbers() {
    assert_eq!(eval("1 != 2"), Value::Bool(true));
    assert_eq!(eval("1 != 1"), Value::Bool(false));
}

#[test]
fn strict_ne() {
    assert_eq!(eval("1 !== '1'"), Value::Bool(true));
    assert_eq!(eval("1 !== 1"), Value::Bool(false));
}

#[test]
fn lt_gt() {
    assert_eq!(eval("1 < 2"), Value::Bool(true));
    assert_eq!(eval("2 < 1"), Value::Bool(false));
    assert_eq!(eval("2 > 1"), Value::Bool(true));
    assert_eq!(eval("1 > 2"), Value::Bool(false));
}

#[test]
fn lte_gte() {
    assert_eq!(eval("1 <= 1"), Value::Bool(true));
    assert_eq!(eval("1 <= 2"), Value::Bool(true));
    assert_eq!(eval("2 <= 1"), Value::Bool(false));
    assert_eq!(eval("1 >= 1"), Value::Bool(true));
    assert_eq!(eval("2 >= 1"), Value::Bool(true));
    assert_eq!(eval("1 >= 2"), Value::Bool(false));
}

// ─── Logical ───

#[test]
fn not_operator() {
    assert_eq!(eval("!true"), Value::Bool(false));
    assert_eq!(eval("!false"), Value::Bool(true));
    assert_eq!(eval("!0"), Value::Bool(true));
    assert_eq!(eval("!1"), Value::Bool(false));
    assert_eq!(eval("!''"), Value::Bool(true));
    assert_eq!(eval("!null"), Value::Bool(true));
    assert_eq!(eval("!undefined"), Value::Bool(true));
}

#[test]
fn truthiness() {
    assert_eq!(eval("!!1"), Value::Bool(true));
    assert_eq!(eval("!!0"), Value::Bool(false));
    assert_eq!(eval("!!'a'"), Value::Bool(true));
    assert_eq!(eval("!!''"), Value::Bool(false));
    assert_eq!(eval("!!null"), Value::Bool(false));
}

// ─── typeof ───

#[test]
fn typeof_number() {
    assert_eq!(eval("typeof 42").to_string_value(), "number");
}

#[test]
fn typeof_string() {
    assert_eq!(eval("typeof 'hello'").to_string_value(), "string");
}

#[test]
fn typeof_bool() {
    assert_eq!(eval("typeof true").to_string_value(), "boolean");
}

#[test]
fn typeof_null() {
    assert_eq!(eval("typeof null").to_string_value(), "object");
}

#[test]
fn typeof_undefined() {
    assert_eq!(eval("typeof undefined").to_string_value(), "undefined");
}

// ─── String operations ───

#[test]
fn string_concat() {
    assert_eq!(
        eval("'hello' + ' ' + 'world'").to_string_value(),
        "hello world"
    );
}

#[test]
fn string_number_concat() {
    assert_eq!(eval("'age: ' + 42").to_string_value(), "age: 42");
    assert_eq!(eval("1 + ' items'").to_string_value(), "1 items");
}

// ─── Variables ───

#[test]
fn var_decl_and_use() {
    assert_eq!(eval("var x = 10; x").to_number(), 10.0);
}

#[test]
fn var_reassign() {
    assert_eq!(eval("var x = 1; x = 2; x").to_number(), 2.0);
}

#[test]
fn var_undefined_init() {
    assert_eq!(eval("var x; x"), Value::Undefined);
}

#[test]
fn multiple_vars() {
    assert_eq!(eval("var a = 1; var b = 2; a + b").to_number(), 3.0);
}

// ─── Control flow ───

#[test]
fn if_true_branch() {
    assert_eq!(eval("var x = 0; if (true) { x = 1; } x").to_number(), 1.0);
}

#[test]
fn if_false_branch() {
    assert_eq!(eval("var x = 0; if (false) { x = 1; } x").to_number(), 0.0);
}

#[test]
fn if_else() {
    assert_eq!(
        eval("var x = 0; if (false) { x = 1; } else { x = 2; } x").to_number(),
        2.0
    );
}

#[test]
fn if_else_if() {
    assert_eq!(
        eval("var x = 0; if (1 < 0) { x = 1; } else if (1 > 0) { x = 2; } else { x = 3; } x")
            .to_number(),
        2.0
    );
}

#[test]
fn while_loop() {
    assert_eq!(
        eval("var i = 0; var sum = 0; while (i < 5) { sum = sum + i; i = i + 1; } sum").to_number(),
        10.0
    );
}

#[test]
fn for_loop() {
    assert_eq!(
        eval("var sum = 0; for (var i = 0; i < 5; i = i + 1) { sum = sum + i; } sum").to_number(),
        10.0
    );
}

#[test]
fn nested_if() {
    assert_eq!(
        eval("var x = 0; if (true) { if (true) { x = 1; } } x").to_number(),
        1.0
    );
}

// ─── Functions (R1: function call frames not yet wired) ───

#[test]
#[ignore]
fn function_declaration() {
    assert_eq!(
        eval("function add(a, b) { return a + b; } add(2, 3)").to_number(),
        5.0
    );
}

#[test]
fn function_no_return() {
    assert_eq!(eval("function noop() { 1; } noop()"), Value::Undefined);
}

#[test]
#[ignore]
fn function_closure() {
    assert_eq!(
        eval("function make_adder(n) { return function(x) { return x + n; }; } var add5 = make_adder(5); add5(3)").to_number(),
        8.0
    );
}

// ─── Exceptions (R1: try/catch/finally not yet wired) ───

#[test]
#[ignore]
fn throw_and_catch() {
    assert_eq!(
        eval("var caught = 0; try { throw 42; } catch (e) { caught = e; } caught").to_number(),
        42.0
    );
}

#[test]
#[ignore]
fn catch_string() {
    assert_eq!(
        eval("var msg = ''; try { throw 'error!'; } catch (e) { msg = e; } msg").to_string_value(),
        "error!"
    );
}

#[test]
#[ignore]
fn try_finally() {
    assert_eq!(
        eval("var x = 0; try { x = 1; } finally { x = 2; } x").to_number(),
        2.0
    );
}

#[test]
#[ignore]
fn try_catch_finally() {
    assert_eq!(
        eval("var x = 0; try { throw 1; } catch (e) { x = e; } finally { x = x + 10; } x")
            .to_number(),
        11.0
    );
}

#[test]
fn uncaught_throws() {
    assert!(eval_err("throw 'boom'").contains("boom"));
}

// ─── Edge cases ───

#[test]
fn empty_program() {
    assert_eq!(eval(""), Value::Undefined);
}

#[test]
fn single_number() {
    assert_eq!(eval("42").to_number(), 42.0);
}

#[test]
fn complex_expression() {
    assert_eq!(
        eval("var a = 2; var b = 3; var c = 4; (a + b) * c - a").to_number(),
        18.0
    );
}

#[test]
fn string_comparison() {
    assert_eq!(eval("'abc' === 'abc'"), Value::Bool(true));
    assert_eq!(eval("'abc' === 'def'"), Value::Bool(false));
}

#[test]
fn floating_point() {
    assert!((eval("0.1 + 0.2").to_number() - 0.3).abs() < 1e-10);
}

#[test]
fn negative_numbers() {
    assert_eq!(eval("-5 + 3").to_number(), -2.0);
    assert_eq!(eval("5 + -3").to_number(), 2.0);
}

#[test]
fn chained_comparison() {
    assert_eq!(eval("1 < 2"), Value::Bool(true));
    assert_eq!(eval("2 > 1"), Value::Bool(true));
}

#[test]
fn boolean_arithmetic() {
    assert_eq!(eval("true + true").to_number(), 2.0);
    assert_eq!(eval("true * 10").to_number(), 10.0);
    assert_eq!(eval("false + 5").to_number(), 5.0);
}

#[test]
fn null_arithmetic() {
    assert_eq!(eval("null + 1").to_number(), 1.0);
    assert_eq!(eval("null + null").to_number(), 0.0);
}

#[test]
fn undefined_arithmetic() {
    assert!(eval("undefined + 1").to_number().is_nan());
}

#[test]
fn deeply_nested_parens() {
    assert_eq!(eval("((((1 + 2))))").to_number(), 3.0);
}

#[test]
fn long_addition_chain() {
    assert_eq!(
        eval("1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10").to_number(),
        55.0
    );
}

#[test]
fn mixed_operators() {
    assert_eq!(eval("2 + 3 * 4 - 1").to_number(), 13.0);
    assert_eq!(eval("(2 + 3) * (4 - 1)").to_number(), 15.0);
}

#[test]
fn semicolons_optional_at_end() {
    assert_eq!(eval("42").to_number(), 42.0);
    assert_eq!(eval("42;").to_number(), 42.0);
}

#[test]
fn multiple_statements() {
    assert_eq!(
        eval("var a = 1; var b = 2; var c = 3; a + b + c").to_number(),
        6.0
    );
}

#[test]
fn string_length_via_typeof() {
    assert_eq!(eval("typeof 'hello'").to_string_value(), "string");
    assert_eq!(eval("typeof ''").to_string_value(), "string");
}

#[test]
fn comparison_chains() {
    assert_eq!(eval("1 < 2 < 3"), Value::Bool(true));
    assert_eq!(eval("3 > 2 > 1"), Value::Bool(false));
}

#[test]
fn void_operator() {
    assert_eq!(eval("void 0"), Value::Undefined);
    assert_eq!(eval("void 42"), Value::Undefined);
}

#[test]
#[ignore]
fn nested_try_catch() {
    assert_eq!(
        eval(
            "var x = 0; try { try { throw 1; } catch (e) { throw e + 1; } } catch (e) { x = e; } x"
        )
        .to_number(),
        2.0
    );
}

#[test]
#[ignore]
fn while_with_break() {
    assert_eq!(
        eval("var i = 0; while (i < 10) { if (i === 5) { break; } i = i + 1; } i").to_number(),
        5.0
    );
}

#[test]
#[ignore]
fn for_with_continue() {
    assert_eq!(
        eval("var sum = 0; for (var i = 0; i < 10; i = i + 1) { if (i % 2 === 0) { continue; } sum = sum + i; } sum").to_number(),
        25.0
    );
}

#[test]
fn large_number() {
    assert_eq!(eval("1000000").to_number(), 1000000.0);
    assert_eq!(eval("1e6").to_number(), 1000000.0);
}

#[test]
fn zero_division() {
    assert_eq!(eval("1 / 0").to_number(), f64::INFINITY);
    assert_eq!(eval("-1 / 0").to_number(), f64::NEG_INFINITY);
}

#[test]
fn string_escape_sequences() {
    assert_eq!(eval("'\\n'").to_string_value(), "\n");
    assert_eq!(eval("'\\t'").to_string_value(), "\t");
    assert_eq!(eval("'\\\\'").to_string_value(), "\\");
    assert_eq!(eval("'\\''").to_string_value(), "'");
}
