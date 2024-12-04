use super::{vector::Vector, nums::abs};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
    where T: Default + Copy
{
    fn default() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn iter(&self) -> impl Iterator<Item = &[T; N]> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T; N]> {
        self.data.iter_mut()
    }

    pub fn into_inner(self) -> [[T; N]; M] {
        self.data
    }

    pub fn concat<const M2: usize>(&self, other: &Matrix<T, M2, N>) -> Matrix<T, {M + M2}, N>
        where T: Copy + Default
    {
        let mut result: Matrix<T, {M+M2}, N> = Matrix::default();

        for i in 0..M {
            for j in 0..N {
                result[i][j] = self[i][j];
            }
        }

        for i in 0..M2 {
            for j in 0..N {
                result[i + M][j] = other[i][j];
            }
        }

        result
    }
}

impl<T, const M: usize, const N: usize> From<Matrix<T, M, N>> for [[T; N]; M] {
    fn from(val: Matrix<T, M, N>) -> Self {
        val.data
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from(val: [[T; N]; M]) -> Self {
        Self { data: val }
    }
}

impl<T, const N: usize> From<Vector<T, N>> for Matrix<T, N, 1>
    where T: Default + Copy
{
    fn from(val: Vector<T, N>) -> Self {
        let mut data = [[T::default(); 1]; N];
        for i in 0..N {
            data[i][0] = val[i];
        }
        Self { data }
    }
}

impl<T, const M: usize, const N: usize> std::ops::Index<usize> for Matrix<T, M, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const M: usize, const N: usize> std::ops::IndexMut<usize> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const M: usize, const N: usize> std::ops::Add<Matrix<T, M, N>> for Matrix<T, M, N>
    where T: Default + Copy + std::ops::Add<Output = T>
{
    type Output = Matrix<T, M, N>;

    fn add(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = Matrix::default();

        for i in 0..M {
            for j in 0..N {
                result[i][j] = self[i][j] + rhs[i][j];
            }
        }

        result
    }
}

impl<T, const M: usize, const N: usize> std::ops::Sub<Matrix<T, M, N>> for Matrix<T, M, N>
    where T: Default + Copy + std::ops::Sub<Output = T>
{
    type Output = Matrix<T, M, N>;

    fn sub(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = Matrix::default();

        for i in 0..M {
            for j in 0..N {
                result[i][j] = self[i][j] - rhs[i][j];
            }
        }

        result
    }
}

impl<T, const M: usize, const N: usize> std::ops::Mul<Vector<T, N>> for Matrix<T, M, N>
    where T: Default + Copy + std::ops::Mul<Output = T> + std::ops::AddAssign<T>
{
    type Output = Vector<T, M>;

    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = [T::default(); M];

        #[allow(clippy::needless_range_loop)]
        for i in 0..M {
            for j in 0..N {
                result[i] += self.data[i][j] * rhs[j];
            }
        }

        result.into()
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
    where T: Default + Copy
{
    /// Returns the transpose of this matrix.
    /// 
    /// # Examples
    /// ```
    /// use aoc2023::helpers::matrix::Matrix;
    /// 
    /// let matrix: Matrix<_, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
    /// let transpose = matrix.transpose();
    /// assert_eq!(transpose.into(), [[1, 4], [2, 5], [3, 6]]);
    /// ```
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut result = Matrix {
            data: [[T::default(); M]; N],
        };

        for i in 0..M {
            for j in 0..N {
                result.data[j][i] = self.data[i][j];
            }
        }

        result
    }
}

impl<T, const M: usize> Matrix<T, M, { M + 1 }>
    where T: std::fmt::Display + std::cmp::PartialOrd + Copy + Default + std::ops::Neg<Output = T> + std::ops::Div<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>
{
    /**
        The following code is based on code written by The Algorithms authors and contributors,
        which is licensed under the MIT license (reproduced below).

        MIT License

        Copyright (c) 2019 The Algorithms

        Permission is hereby granted, free of charge, to any person obtaining a copy
        of this software and associated documentation files (the "Software"), to deal
        in the Software without restriction, including without limitation the rights
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
        copies of the Software, and to permit persons to whom the Software is
        furnished to do so, subject to the following conditions:

        The above copyright notice and this permission notice shall be included in all
        copies or substantial portions of the Software.

        THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
        SOFTWARE.
        */
    pub fn gauss_jordan_elimination(&mut self) -> Result<Vector<T, M>, Box<dyn std::error::Error>>
    {
        let mut mpivot = 0;
        let mut npivot = 0;

        while mpivot < M && npivot < { M + 1 } {
            let mut v_max = abs(self.data[mpivot][npivot]);
            let mut i_max = mpivot;
            for i in mpivot + 1..M {
                if abs(self.data[i][npivot]) > v_max {
                    i_max = i;
                    v_max = abs(self.data[i][npivot]);
                }
            }

            if self.data[i_max][npivot] == T::default() {
                npivot += 1;
            } else {
                self.data.swap(mpivot, i_max);

                for i in mpivot + 1..M {
                    let factor = self.data[i][npivot] / self.data[mpivot][npivot];
                    self.data[i][npivot] = T::default();
                    for j in (npivot+1)..(M + 1) {
                        self.data[i][j] = self.data[i][j] - factor * self.data[mpivot][j];
                    }
                }

                mpivot += 1;
                npivot += 1;
            }
        }

        for i in (1..M).rev() {
            self.eliminate(i);
        }

        // Disable cargo clippy warnings about needless range loops.
        // Checking the diagonal like this is simpler than any alternative.
        #[allow(clippy::needless_range_loop)]
        for i in 0..M {
            if self.data[i][i] == T::default() {
                println!("{}", self);
                return Err("Infinitely many solutions or no solutions.".into());
            }
        }

        let mut result: Vector<T, M> = Vector::default();
        for i in 0..M {
            result[i] = self.data[i][M] / self.data[i][i];
        }

        Ok(result)
    }

    fn eliminate(&mut self, i: usize)
    {
        let size = self.data.len();
        if self.data[i][i] == T::default() {
        } else {
            for j in (1..i + 1).rev() {
                let factor = self.data[j - 1][i] / self.data[i][i];
                for k in (0..size + 1).rev() {
                    self.data[j - 1][k] = self.data[j - 1][k] - factor * self.data[i][k];
                }
            }
        }
    }
}


impl<T, const M: usize, const N: usize> std::fmt::Debug for Matrix<T, M, N>
    where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix([")?;
        for row in self.data.iter() {
            writeln!(f, "  {:?},", row)?;
        }
        write!(f, "])")
    }
}

impl<T, const M: usize, const N: usize> std::fmt::Display for Matrix<T, M, N>
    where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.iter() {
            for (i, val) in row.iter().enumerate() {
                write!(f, "{:5.1}", val)?;

                if i < N - 1 {
                    write!(f, " ")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;
    use super::Matrix;

    #[test]
    fn test_gauss() {
        let mut matrix: Matrix<f32, 6, 7> = [
            [1.5, 2.0, 1.0, -1.0, -2.0, 1.0, 1.0],
            [3.0, 3.0, -1.0, 16.0, 18.0, 1.0, 1.0],
            [1.0, 1.0, 3.0, -2.0, -6.0, 1.0, 1.0],
            [1.0, 1.0, 99.0, 19.0, 2.0, 1.0, 1.0],
            [1.0, -2.0, 16.0, 1.0, 9.0, 10.0, 1.0],
            [1.0, 3.0, 1.0, -5.0, 1.0, 1.0, 95.0],
        ].into();

        let result:  Vector<f32, 6> = [
            -264.05887, 159.63197, -6.156916, 35.310352, -18.806673, 81.67833,
        ].into();
        
        assert_eq!(matrix.gauss_jordan_elimination().unwrap(), result);
    }
}