use super::matrix::Matrix;

/// Represents a fixed size N-dimensional vector.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Default for Vector<T, N>
    where T: Default + Copy
{
    fn default() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T> Vector<T, 3>
    where T: Copy + Default + std::ops::Neg<Output = T>
{
    /// Returns the cross-matrix representation of this vector.
    /// 
    /// # Examples
    /// ```
    /// use aoc2023::helpers::matrix::Vector;
    /// 
    /// let vector: Vector<_, _> = [1, 2, 3].into();
    /// let cross_matrix = vector.cross_matrix();
    /// assert_eq!(cross_matrix.into(), [[0, -3, 2], [3, 0, -1], [-2, 1, 0]]);
    /// ```
    pub fn cross_matrix(&self) -> Matrix<T, 3, 3> {
        [
            [T::default(), -self.data[2], self.data[1]],
            [self.data[2], T::default(), -self.data[0]],
            [-self.data[1], self.data[0], T::default()],
        ].into()
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(val: [T; N]) -> Self {
        Self { data: val }
    }
}

impl<T, const N: usize> From<Vector<T, N>> for [T; N] {
    fn from(val: Vector<T, N>) -> Self {
        val.data
    }
}

impl<T, const N: usize> std::ops::Add<Vector<T, N>> for Vector<T, N>
    where T: Copy + Default + std::ops::Add<Output = T>
{
    type Output = Vector<T, N>;

    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = Vector {
            data: [T::default(); N],
        };

        for i in 0..N {
            result.data[i] = self.data[i] + rhs.data[i];
        }

        result
    }
}

impl<T, const N: usize> std::ops::Sub<Vector<T, N>> for Vector<T, N>
    where T: Copy + Default + std::ops::Sub<Output = T>
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = Vector {
            data: [T::default(); N],
        };

        for i in 0..N {
            result.data[i] = self.data[i] - rhs.data[i];
        }

        result
    }
}

impl<T, const N: usize> std::ops::Neg for Vector<T, N>
    where T: Copy + Default + std::ops::Neg<Output = T>
{
    type Output = Vector<T, N>;

    fn neg(self) -> Self::Output {
        let mut result = Vector {
            data: [T::default(); N],
        };

        for i in 0..N {
            result.data[i] = -self.data[i];
        }

        result
    }
}

impl<T> std::ops::Mul<Vector<T, 3>> for Vector<T, 3>
    where T: Copy + Default + std::ops::Neg<Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign<T>
{
    type Output = Vector<T, 3>;

    fn mul(self, rhs: Vector<T, 3>) -> Self::Output {
        self.cross_matrix() * rhs
    }
}

impl<T, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> std::fmt::Debug for Vector<T, N>
    where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector").field("data", &self.data).finish()
    }
}

impl<T, const N: usize> std::fmt::Display for Vector<T, N>
    where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;

        for i in 0..N {
            write!(f, "{}", self.data[i])?;

            if i < N - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ">")
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn test_add() {
        let a: Vector<_, 3> = [1, 2, 3].into();
        let b: Vector<_, 3> = [4, 5, 6].into();
        let c: Vector<_, 3> = [5, 7, 9].into();

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a: Vector<_, 3> = [1, 2, 3].into();
        let b: Vector<_, 3> = [4, 5, 6].into();
        let c: Vector<_, 3> = [-3, -3, -3].into();

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_neg() {
        let a: Vector<_, 3> = [1, 2, 3].into();
        let b: Vector<_, 3> = [-1, -2, -3].into();

        assert_eq!(-a, b);
    }

    #[test]
    fn test_mul() {
        let a: Vector<_, 3> = [1, 2, 3].into();
        let b: Vector<_, 3> = [1, 5, 7].into();
        let c: Vector<_, 3> = [-1, -4, 3].into();

        assert_eq!(a * b, c);
    }
}