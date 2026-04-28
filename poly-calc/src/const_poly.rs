//use num_traits::zero;

use crate::util;

#[cfg(test)]
mod test;

/// A polynomial
///
/// If f(x) is `Poly::new(coeff)` and c<sub>i</sub> is `coeff[i]`,
/// then f(x) = &Sigma; c<sub>i</sub> x<sup>i</sup>
#[derive(Clone, Copy, Debug)]
pub struct Poly<const N: usize> {
    coeff: [f64; N],
}

impl<const N: usize> Poly<N> {
    pub const fn new(coeff: [f64; N]) -> Self {
        Self { coeff }
    }

    pub const fn zero() -> Self {
        Self::new([0.0; N])
    }

    pub const fn one() -> Self {
        let mut coeff = [0.0; N];
        coeff[0] = 1.0;
        Self::new(coeff)
    }

    pub const fn len() -> usize {
        N
    }

    /// Gets the stored coefficient or returns 0.0 if coefficient `i` does not exist
    pub const fn get_coeff(&self, i: usize) -> f64 {
        if i < N { self.coeff[i] } else { 0.0 }
    }

    /// Stores the `value` to coefficient `i`
    /// # Panics
    /// Panics if attempting to store a non-zero `value` to a coefficient `i` that does not exist
    pub const fn set_coeff(&mut self, i: usize, value: f64) {
        if i < N {
            self.coeff[i] = value;
        } else if value != 0.0 {
            panic!(
                "attempting to store a non-zero `value` to a coefficient `i` that does not exist"
            );
        }
    }

    pub const fn eq<const L: usize>(&self, rhs: &Poly<L>) -> bool {
        let max_len = util::max(N, L);
        let mut eq = true;
        let mut i = 0;
        while i < max_len {
            let lhs = self.get_coeff(i);
            let rhs = rhs.get_coeff(i);
            eq &= lhs == rhs;
            i += 1;
        }
        eq
    }

    /// Evaluates the polynomial at `x`.
    /// Evaluates using Horner's rule; i.e.,
    /// f(x) = ((c<sub>n</sub> x + c<sub>n-1</sub>) x + ... ) x + c<sub>0</sub>
    pub const fn eval(&self, x: f64) -> f64 {
        if x == 0.0 {
            return self.get_coeff(0);
        }
        let mut acc = 0.0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            acc = (acc * x) + self.coeff[i];
        }
        acc
    }

    /// Calculates the deritative of `self`
    /// # Panics
    /// Panics if the deritative length `M` is not long enough.
    /// For `Poly<N>`, `M` should be `N - 1`.
    pub const fn diff<const M: usize>(&self) -> Poly<M> {
        let mut diff = Poly::zero();
        let mut i = 1;
        while i < N {
            diff.set_coeff(i - 1, (i as f64) * self.coeff[i]);
            i += 1;
        }
        diff
    }

    /// Calculates the integral of `self` from 0.
    /// # Panics
    /// Panics if the integral length `M` is not long enough.
    /// For `Poly<N>`, `M` should be `N + 1`.
    pub const fn int<const M: usize>(&self) -> Poly<M> {
        let mut int = Poly::zero();
        let mut i = 0;
        let mut n = 1.0;
        while i < N {
            int.set_coeff(i + 1, self.coeff[i] / n);
            i += 1;
            n += 1.0;
        }
        int
    }

    /// Calculates the definite integral of `self` from 0 to 1.
    pub const fn int_on_unit(&self) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;
        let mut n = 1.0;
        while i < N {
            sum += self.coeff[i] / n;
            i += 1;
            n += 1.0;
        }
        sum
    }

    /// Calculates the definite integral of `self` from 0.
    pub const fn int_from_zero(&self, x: f64) -> f64 {
        let mut acc = 0.0;
        let mut i = N;
        let mut n = N as f64;
        while i > 0 {
            i -= 1;
            let coeff = self.coeff[i] / n;
            acc = (acc * x) + coeff;
            n -= 1.0;
        }
        acc
    }

    /// Calculates the definite integral of `self` from `lower` to `upper`.
    pub const fn def_int(&self, lower: f64, upper: f64) -> f64 {
        let mut acc_lower = 0.0;
        let mut acc_upper = 0.0;
        let mut i = N;
        let mut n = N as f64;
        while i > 0 {
            i -= 1;
            let coeff = self.coeff[i] / n;
            acc_upper = (acc_upper * upper) + coeff;
            acc_lower = (acc_lower * lower) + coeff;
            n -= 1.0;
        }
        acc_upper - acc_lower
    }

    /// Adds two polynomials
    /// # Panics
    /// Panics if the return length is not long enough
    pub const fn add<const L: usize, const M: usize>(&self, rhs: &Poly<L>) -> Poly<M> {
        let max_len = util::max(N, L);
        let mut sum = Poly::zero();
        let mut i = 0;
        while i < max_len {
            let lhs = self.get_coeff(i);
            let rhs = rhs.get_coeff(i);
            sum.set_coeff(i, lhs + rhs);
            i += 1;
        }
        sum
    }

    /// Adds a polynomial to `self`
    /// # Panics
    /// Panics if `rhs` has non-zero coefficients that do not fit in `self`
    pub const fn add_assign<const M: usize>(&mut self, rhs: &Poly<M>) {
        let mut i = 0;
        while i < M {
            let lhs = self.get_coeff(i);
            let rhs = rhs.coeff[i];
            self.set_coeff(i, lhs + rhs);
            i += 1;
        }
    }

    /// Subtracts two polynomials
    /// # Panics
    /// Panics if the return length is not long enough
    pub const fn sub<const L: usize, const M: usize>(&self, rhs: &Poly<L>) -> Poly<M> {
        let max_len = util::max(N, L);
        let mut diff = Poly::zero();
        let mut i = 0;
        while i < max_len {
            let lhs = self.get_coeff(i);
            let rhs = rhs.get_coeff(i);
            diff.set_coeff(i, lhs - rhs);
            i += 1;
        }
        diff
    }

    /// Multiplies two polynomials
    /// # Panics
    /// Panics if the return length is not long enough
    pub const fn mul<const L: usize, const M: usize>(&self, rhs: &Poly<L>) -> Poly<M> {
        let mut sum = [0.0; M];
        let mut i = 0;
        while i < N {
            let lhs = self.coeff[i];
            if lhs == 0.0 {
                i += 1;
                continue;
            }
            let mut j = 0;
            while j < L {
                let rhs = rhs.coeff[j];
                let prod = lhs * rhs;
                if prod != 0.0 {
                    sum[i + j] += prod;
                }
                j += 1;
            }
            i += 1;
        }
        Poly::new(sum)
    }

    pub const fn scale(&mut self, factor: f64) -> &mut Self {
        let mut i = 0;
        while i < N {
            self.coeff[i] *= factor;
            i += 1;
        }
        self
    }

    /// Returns a change of variable on `self`.  
    /// If `self` is f(x) and `x` is x(u),
    /// then `self.change(x)` returns f(x(u)).
    ///
    /// Using Horner's rule; i.e.,
    /// f(x(u)) = ((c<sub>n</sub> x(u) + c<sub>n-1</sub>) x(u) + ... ) x(u) + c<sub>0</sub>
    pub const fn change<const U: usize, const M: usize>(&self, u: &Poly<U>) -> Poly<M> {
        let mut acc = Poly::zero();
        let mut i = N;
        while i > 0 {
            i -= 1;
            acc = acc.mul(u);
            acc.coeff[0] += self.coeff[i];
        }
        acc
    }
}

/// A polynomial with two variables
///
/// If f(x, y) is `Poly2d::new(coeff)` and c<sub>ij</sub> is `coeff[i][j]`,
/// then f(x, y) = &Sigma; c<sub>ij</sub> x<sup>i</sup> y<sup>j</sup>
pub struct Poly2d<const NX: usize, const NY: usize> {
    coeff: [[f64; NY]; NX], // Coefficients
}

impl<const NX: usize, const NY: usize> Poly2d<NX, NY> {
    pub const fn new(coeff: [[f64; NY]; NX]) -> Self {
        Poly2d { coeff }
    }

    pub const fn zero() -> Self {
        Poly2d {
            coeff: [[0.0; NY]; NX],
        }
    }
    pub const fn one() -> Self {
        let mut coeff = [[0.0; NY]; NX];
        coeff[0][0] = 1.0;
        Self::new(coeff)
    }

    /// Gets the stored coefficient or returns 0.0 if coefficient `i, j` does not exist
    pub const fn get_coeff(&self, i: usize, j: usize) -> f64 {
        if i < NX && j < NY {
            self.coeff[i][j]
        } else {
            0.0
        }
    }

    /// Gets the stored coefficient or returns 0.0 if coefficient `i, j` does not exist
    pub const fn get_mut_coeff(&mut self, i: usize, j: usize) -> &mut f64 {
        if i < NX && j < NY {
            &mut self.coeff[i][j]
        } else {
            panic!(
                "attempting to get a mutable reference to a coefficient `i, j` that does not exist"
            );
        }
    }

    /// Stores `value` to coefficient `i, j`
    /// # Panics
    /// Panics if attempting to store a non-zero `value` to a coefficient `i, j` that does not exist
    pub const fn set_coeff(&mut self, i: usize, j: usize, value: f64) {
        if i < NX && j < NY {
            self.coeff[i][j] = value;
        } else if value != 0.0 {
            panic!(
                "attempting to store a non-zero `value` to a coefficient `i, j` that does not exist"
            );
        }
    }

    pub const fn diff_x<const S: usize>(&self) -> Poly2d<S, NY> {
        let mut diff = Poly2d::zero();
        let mut i = 1;
        while i < NX {
            let mut j = 0;
            while j < NY {
                diff.set_coeff(i - 1, j, (i as f64) * self.coeff[i][j]);
                j += 1;
            }
            i += 1;
        }
        diff
    }

    pub const fn diff_y<const S: usize>(&self) -> Poly2d<NX, S> {
        let mut diff = Poly2d::zero();
        let mut i = 0;
        while i < NX {
            let mut j = 1;
            while j < NY {
                diff.set_coeff(i, j - 1, (j as f64) * self.coeff[i][j]);
                j += 1;
            }
            i += 1;
        }
        diff
    }

    pub const fn int_x<const S: usize>(&self) -> Poly2d<S, NY> {
        let mut int = Poly2d::zero();
        let mut i = 0;
        while i < NX {
            let mut j = 0;
            while j < NY {
                int.set_coeff(i + 1, j, self.coeff[i][j] / ((i + 1) as f64));
                j += 1;
            }
            i += 1;
        }
        int
    }

    pub const fn int_y<const S: usize>(&self) -> Poly2d<NX, S> {
        let mut int = Poly2d::zero();
        let mut i = 0;
        while i < NX {
            let mut j = 0;
            while j < NY {
                int.set_coeff(i, j + 1, self.coeff[i][j] / ((j + 1) as f64));
                j += 1;
            }
            i += 1;
        }
        int
    }

    /// Uses Horner's rule; i.e.,
    /// f(x, y) = ((c<sub>n</sub>(y) x + c<sub>n-1</sub>(y)) x + ... ) x + c<sub>0</sub>(y) where,
    /// c<sub>i</sub>(y) = ((c<sub>i,n</sub> y + c<sub>i,n-1</sub>) y + ... ) y + c<sub>i,0</sub>
    pub const fn eval(&self, x: f64, y: f64) -> f64 {
        // sum over i, j of c_ij * x^i * y^j = sum over i of c_i * x^i
        let mut acc = 0.0;
        let mut i = NX;
        while i > 0 {
            i -= 1;
            let mut j = NY;
            let mut c_i = 0.0; // c_i = sum over j of c_ij * y^j
            while j > 0 {
                j -= 1;
                c_i = (c_i * y) + self.coeff[i][j];
            }
            acc = (acc * x) + c_i;
        }
        acc
    }

    /// If `self` is f(x, y) and `x` is x(y),
    /// then `self.eval_x(x)` returns f(x(y), y).
    ///
    /// This uses Horner's rule; i.e.,
    /// f(x(y), y) = ((c<sub>n</sub>(y) x(y) + c<sub>n-1</sub>(y)) x(y) + ... ) x(y) + c<sub>0</sub>(y) where,
    /// c<sub>i</sub>(y) = &Sigma; c<sub>ij</sub> y<sup>j</sup>
    pub const fn eval_x<const S: usize, const T: usize>(&self, x: &Poly<S>) -> Poly<T> {
        // sum over i, j of c_ij * x^i * y^j = sum over i of c_i * x^i
        let mut acc = Poly::<T>::zero();
        let mut i = NX;
        while i > 0 {
            i -= 1;
            let c_i = Poly::new(self.coeff[i]); // c_i = c_ij * y^j
            acc = acc.mul(x);
            acc.add_assign(&c_i);
        }
        acc
    }

    /// If `self` is f(x, y) and `y` is y(x),
    /// then `self.eval_y(y)` returns f(x, y(x)).
    ///
    /// This uses Horner's rule; i.e.,
    /// f(x, y(x)) = ((c<sub>n</sub>(x) y(x) + c<sub>n-1</sub>(x)) y(x) + ... ) y(x) + c<sub>0</sub>(x) where,
    /// c<sub>j</sub>(x) = &Sigma; c<sub>ij</sub> x<sup>i</sup>
    pub const fn eval_y<const S: usize, const T: usize>(&self, y: &Poly<S>) -> Poly<T> {
        // sum over i, j of c_ij * x^i * y^j = sum over j of c_j * y^j
        let mut acc = Poly::zero();
        let mut j = NY;
        while j > 0 {
            j -= 1;
            let mut c_j = Poly::<NX>::zero();
            let mut i = 0;
            while i < NX {
                c_j.coeff[i] = self.coeff[i][j]; // c_j = c_ij * x^i
                i += 1;
            }
            acc = acc.mul(y);
            acc.add_assign(&c_j);
        }
        acc
    }

    /// Computes the definite integral of `self` with respect to x from `lower` to `upper`.
    /// # Panics
    /// Panics if the integral length `I` or the output length `O` is not long enough.
    /// For `Poly2d<NX, NY>`, `I` should be `NX + 1`.
    /// `O` should be `NX * (L - 1) + 1` or `NX * (U - 1) + 1`, whichever is larger.
    pub const fn def_int_x<const I: usize, const L: usize, const U: usize, const O: usize>(
        &self,
        lower: &Poly<L>,
        upper: &Poly<U>,
    ) -> Poly<O> {
        let int = self.int_x::<I>();
        int.eval_x::<U, O>(upper).sub(&int.eval_x::<L, O>(lower))
    }

    /// Computes the definite integral of `self` with respect to y from `lower` to `upper`.
    /// # Panics
    /// Panics if the integral length `I` or the output length `O` is not long enough.
    /// For `Poly2d<NX, NY>`, `I` should be `NY + 1`.
    /// `O` should be `NY * (L - 1) + 1` or `NY * (U - 1) + 1`, whichever is larger.
    pub const fn def_int_y<const I: usize, const L: usize, const U: usize, const O: usize>(
        &self,
        lower: &Poly<L>,
        upper: &Poly<U>,
    ) -> Poly<O> {
        let int = self.int_y::<I>();
        int.eval_y::<U, O>(upper).sub(&int.eval_y::<L, O>(lower))
    }

    pub const fn jacobian<const XU: usize, const XV: usize, const YU: usize, const YV: usize>(
        x: &Poly2d<XU, XV>,
        y: &Poly2d<YU, YV>,
    ) -> Poly2d<NX, NY> {
        let term_1: Poly2d<NX, NY> = x.diff_x::<XU>().mul(&y.diff_y::<YV>());
        let term_2: Poly2d<NX, NY> = y.diff_x::<YU>().mul(&x.diff_y::<XV>());
        term_1.sub(&term_2)
    }

    /// Integrates the polynomial over a unit triangle,
    /// with vertices at (0, 0), (1, 0), and (0, 1).
    pub const fn int_on_unit_triangle(&self) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;
        let mut num_i = 1.0; // i!
        while i < NX {
            let mut j = 0;
            let mut num = num_i; // i! j!
            let mut den = num * ((i + 1) * (i + 2)) as f64; // (i + j + 2)!
            while j < NY {
                sum += self.coeff[i][j] * num / den;
                j += 1;
                num *= j as f64;
                den *= (i + j + 2) as f64;
            }
            i += 1;
            num_i *= i as f64;
        }
        sum
    }

    #[allow(dead_code)]
    const fn int_on_polygon_by_trapezoid<const M: usize, const N: usize>(
        &self,
        polygon: [(f64, f64); N],
    ) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;
        while i < N {
            let mut j = i + 1;
            if j == N {
                j = 0;
            }
            let (x1, y1) = polygon[i];
            let (x2, y2) = polygon[j];
            // (y - y1) / (y2 - y1) = (x - x1) / (x2 - x1)
            let dx = x2 - x1;
            if dx == 0.0 {
                i += 1;
                continue;
            }
            let dy = y2 - y1;
            let m = dy / dx;
            // y - y1 = m * (x - x1)
            // y = m * x + y1 - m * x1
            let y = Poly::new([y1 - x1 * m, m]);
            let int_x_dx: Poly<M> = self.int_y::<M>().eval_y(&y);
            let int: Poly<M> = int_x_dx.int();
            sum -= int.eval(x2) - int.eval(x1);
            i += 1;
        }
        sum
    }

    pub const fn add<const S: usize, const T: usize, const U: usize, const V: usize>(
        &self,
        rhs: &Poly2d<S, T>,
    ) -> Poly2d<U, V> {
        let max_rows = util::max(NX, T);
        let max_cols = util::max(NY, S);
        let mut sum = Poly2d::zero();
        let mut i = 0;
        while i < max_rows {
            let mut j = 0;
            while j < max_cols {
                let lhs = self.get_coeff(i, j);
                let rhs = rhs.get_coeff(i, j);
                sum.set_coeff(i, j, lhs + rhs);
                j += 1;
            }
            i += 1;
        }
        sum
    }

    pub const fn add_assign<const MX: usize, const MY: usize>(&mut self, rhs: &Poly2d<MX, MY>) {
        let mut i = 0;
        while i < NX {
            let mut j = 0;
            while j < NY {
                let rhs = rhs.get_coeff(i, j);
                self.coeff[i][j] += rhs;
                j += 1;
            }
            i += 1;
        }
    }

    pub const fn sub<const S: usize, const T: usize, const U: usize, const V: usize>(
        &self,
        rhs: &Poly2d<S, T>,
    ) -> Poly2d<U, V> {
        let max_rows = util::max(NX, T);
        let max_cols = util::max(NY, S);
        let mut diff = Poly2d::zero();
        let mut i = 0;
        while i < max_rows {
            let mut j = 0;
            while j < max_cols {
                let lhs = self.get_coeff(i, j);
                let rhs = rhs.get_coeff(i, j);
                diff.set_coeff(i, j, lhs - rhs);
                j += 1;
            }
            i += 1;
        }
        diff
    }

    /// Multiplies two polynomials
    /// # Panics
    /// Panics if the return length is not long enough
    pub const fn mul<const S: usize, const T: usize, const U: usize, const V: usize>(
        &self,
        rhs: &Poly2d<S, T>,
    ) -> Poly2d<U, V> {
        let mut sum = Poly2d::zero();
        let mut i_l = 0;
        while i_l < NX {
            let mut j_l = 0;
            while j_l < NY {
                let lhs = self.coeff[i_l][j_l];
                if lhs != 0.0 {
                    let mut i_r = 0;
                    while i_r < S {
                        let mut j_r = 0;
                        while j_r < T {
                            let rhs = rhs.coeff[i_r][j_r];
                            if rhs != 0.0 {
                                let prod = lhs * rhs;
                                let i = i_l + i_r;
                                let j = j_l + j_r;
                                *sum.get_mut_coeff(i, j) += prod;
                            }
                            j_r += 1;
                        }
                        i_r += 1;
                    }
                }
                j_l += 1;
            }
            i_l += 1;
        }
        sum
    }

    pub const fn scale(&mut self, factor: f64) -> &mut Self {
        let mut i = 0;
        while i < NX {
            let mut j = 0;
            while j < NY {
                self.coeff[i][j] *= factor;
                j += 1;
            }
            i += 1;
        }
        self
    }

    pub const fn change<
        const XU: usize,
        const XV: usize,
        const YU: usize,
        const YV: usize,
        const MU: usize,
        const MV: usize,
    >(
        &self,
        x: &Poly2d<XU, XV>,
        y: &Poly2d<YU, YV>,
    ) -> Poly2d<MU, MV> {
        // sum over i, j of c_ij * x^i * y^j = sum over j of c_j * y^j
        let mut acc = Poly2d::<MU, MV>::zero();
        let mut i = NX;
        while i > 0 {
            i -= 1;
            let mut j = NY;
            let mut c_i = Poly2d::<MU, MV>::zero(); // c_i = sum over j of c_ij * y^j
            while j > 0 {
                j -= 1;
                c_i = c_i.mul(y);
                c_i.coeff[0][0] += self.coeff[i][j];
            }
            acc = acc.mul(&x);
            acc.add_assign(&c_i);
        }
        acc
    }
}

impl<const N: usize> Poly2d<N, N> {
    /// Integrates the polynomial over a `polygon`,
    /// with vertices at `[(x1, y1), (x2, y2), ..., (xN, yN)]`.
    pub const fn int_on_polygon<const L: usize>(&self, polygon: [(f64, f64); L]) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;
        while i < L {
            let j = i + 1;
            let j = if j == L { 0 } else { j };
            let (x1, y1) = polygon[i];
            let (x2, y2) = polygon[j];
            let jacobian = x1 * y2 - x2 * y1;
            if jacobian != 0.0 {
                let x = Poly2d::new([[0.0, x2], [x1, 0.0]]); // x = x1 * u + x2 * v
                let y = Poly2d::new([[0.0, y2], [y1, 0.0]]); // y = y1 * u + y2 * v
                let self_u_v: Poly2d<N, N> = self.change(&x, &y);
                sum += jacobian * self_u_v.int_on_unit_triangle();
            }
            i += 1;
        }
        sum
    }
}

impl Poly2d<1, 1> {
    pub const fn from_f64(coeff: f64) -> Self {
        Poly2d::new([[coeff]])
    }

    pub const fn to_f64(self) -> f64 {
        self.coeff[0][0]
    }
}
