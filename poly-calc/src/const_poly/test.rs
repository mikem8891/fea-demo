use super::*;

#[test]
fn eval() {
    const POLY: Poly<2> = Poly::new([1.0, 2.0]);
    assert_eq!(7.0, POLY.eval(3.0));
    //    assert_eq!(4.0, poly.diff_between(3.0..5.0));
}

#[test]
fn diff() {
    const POLY: Poly<3> = Poly::new([0.0, 1.0, 2.0]);
    const DIFF: Poly<2> = Poly::new([1.0, 4.0]);
    const EQ: bool = DIFF.eq::<2>(&POLY.diff());
    assert!(EQ);
    let poly = Poly::new([]);
    assert!(Poly::<5>::zero().eq(&poly.diff::<2>()));
}

#[test]
fn int() {
    const POLY: Poly<3> = Poly::new([0.0, 1.0, 2.0]);
    const INT: Poly<4> = Poly::new([0.0, 0.0, 0.5, 2.0 / 3.0]);
    const EQ: bool = INT.eq(&POLY.int::<5>());
    assert!(EQ);
    assert_eq!(
        INT.eval(3.0) - INT.eval(1.0),
        0.5 * (9.0 - 1.0) + 2.0 / 3.0 * (27.0 - 1.0)
    );
    let poly = Poly::new([]);
    let int = Poly::new([]);
    assert!(int.eq(&poly.int::<5>()));
}

#[test]
fn add_sub() {
    const LHS: Poly<3> = Poly::new([1.0, 2.0, 3.0]);
    const RHS: Poly<2> = Poly::new([1.0, -1.0]);
    const SUM: Poly<4> = LHS.add(&RHS);
    const DIFF: Poly<3> = LHS.sub(&RHS);
    const ADD_ASSIGN: Poly<3> = const {
        let mut lhs = LHS;
        lhs.add_assign(&RHS);
        lhs
    };
    assert!(SUM.eq(&Poly::new([2.0, 1.0, 3.0])));
    assert!(DIFF.eq(&Poly::new([0.0, 3.0, 3.0])));
    assert!(ADD_ASSIGN.eq(&SUM));
}

#[test]
fn mul() {
    const LHS: Poly<3> = Poly::new([1.0, 2.0, 3.0]);
    const RHS: Poly<2> = Poly::new([1.0, -1.0]);
    const PROD: Poly<4> = LHS.mul(&RHS);
    assert!(PROD.eq(&Poly::new([1.0, 1.0, 1.0, -3.0])));
}

#[test]
fn change_of_var() {
    const POLY: Poly<3> = Poly::new([1.0, 2.0, 3.0]);
    const SUB: Poly<2> = Poly::new([1.0, -1.0]);
    const RESULT: Poly<3> = POLY.change(&SUB);
    assert!(RESULT.eq(&Poly::new([6.0, -8.0, 3.0])));
}

#[test]
fn eval2d() {
    // 1 - x - y
    const COEFF: [[f64; 2]; 2] = [[1.0, -1.0], [-1.0, 0.0]];
    const POLY: Poly2d<2, 2> = Poly2d::new(COEFF);
    assert_eq!(1.0, POLY.eval(0.0, 0.0));
    assert_eq!(0.0, POLY.eval(1.0, 0.0));
    assert_eq!(0.0, POLY.eval(0.0, 1.0));
}

#[test]
fn int2d() {
    // 1 - x - y
    const COEFF_1: [[f64; 2]; 2] = [[1.0, -1.0], [-1.0, 0.0]];
    // x
    const COEFF_2: [[f64; 2]; 2] = [[0.0, 0.0], [1.0, 0.0]];
    // y
    const COEFF_3: [[f64; 2]; 2] = [[0.0, 1.0], [0.0, 0.0]];
    const N_1: Poly2d<2, 2> = Poly2d::new(COEFF_1);
    const N_2: Poly2d<2, 2> = Poly2d::new(COEFF_2);
    const N_3: Poly2d<2, 2> = Poly2d::new(COEFF_3);
    const N_11: Poly2d<3, 3> = N_1.mul(&N_1);
    const N_22: Poly2d<3, 3> = N_2.mul(&N_2);
    const N_33: Poly2d<3, 3> = N_3.mul(&N_3);
    const N_12: Poly2d<3, 3> = N_1.mul(&N_2);
    const N_13: Poly2d<3, 3> = N_1.mul(&N_3);
    const N_23: Poly2d<2, 2> = N_2.mul(&N_3);
    const TRIANGLE: [(f64, f64); 3] = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
    const M_11: f64 = N_11.int_on_polygon(TRIANGLE);
    const M_22: f64 = N_22.int_on_polygon(TRIANGLE);
    const M_33: f64 = N_33.int_on_polygon(TRIANGLE);
    const M_12: f64 = N_12.int_on_polygon(TRIANGLE);
    const M_13: f64 = N_13.int_on_polygon(TRIANGLE);
    const M_23: f64 = N_23.int_on_polygon(TRIANGLE);
    assert_eq!(
        [M_11, M_22, M_33, M_12, M_13, M_23],
        [
            1.0 / 12.0,
            1.0 / 12.0,
            1.0 / 12.0,
            1.0 / 24.0,
            1.0 / 24.0,
            1.0 / 24.0,
        ]
    );
}
