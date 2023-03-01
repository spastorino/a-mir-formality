use expect_test::expect;
use formality_macros::test;
use formality_types::{
    grammar::{Binder, Ty, Wc, Wcs},
    parse::{term, term_with},
};

use crate::{program::Program, prove::prove_wc_list};

use super::with_bindings;

/// Simple example program consisting only of two trait declarations.
const PROGRAM: &str = "
    program(
        [],
        [],
        [],
        []
    )
";

#[test]
fn for_t_exists_u() {
    let program: Program = term(PROGRAM);
    let assumptions: Wcs = Wcs::t();
    with_bindings(|u: Ty| {
        let goal: Wc = term_with(&[("U", u)], "for<ty T> equals(T, U)");
        let constraints = crate::prove::prove_wc_list(program, assumptions, goal);
        expect![[r#"
            {
                {},
            }
        "#]] // FIXME: this should be `{}`
        .assert_debug_eq(&constraints);
    });
}