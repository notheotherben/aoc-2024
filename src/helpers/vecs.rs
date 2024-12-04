use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct RectVec<T>(Vec<T>, usize, usize);

impl<T> RectVec<T>
    where T: Default + Clone
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self(vec![T::default(); rows * cols], rows, cols)
    }
}

impl<T> RectVec<T>
    where T: Clone
{
    pub fn filled(val: T, rows: usize, cols: usize) -> Self {
        Self(vec![val; rows * cols], rows, cols)
    }
}

impl<T> RectVec<T>
{
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if col >= self.2 {
            return None;
        }
        
        self.0.get(row * self.2 + col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if col >= self.2 {
            return None;
        }

        self.0.get_mut(row * self.2 + col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if col >= self.2 {
            panic!("Column index out of bounds");
        }
        
        self.0[row * self.2 + col] = value;
    }

    pub fn rows(&self) -> usize {
        self.1
    }

    pub fn cols(&self) -> usize {
        self.2
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.0.chunks(self.2)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.0.chunks_mut(self.2)
    }

    pub fn row(&self, row: usize) -> &[T] {
        &self.0[row * self.2..(row + 1) * self.2]
    }

    pub fn row_mut(&mut self, row: usize) -> &mut [T] {
        &mut self.0[row * self.2..(row + 1) * self.2]
    }
}

impl<T> std::ops::Index<(usize, usize)> for RectVec<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.1 >= self.2 {
            panic!("Column index out of bounds");
        }

        &self.0[index.0 * self.2 + index.1]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for RectVec<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.1 >= self.2 {
            panic!("Column index out of bounds");
        }

        &mut self.0[index.0 * self.2 + index.1]
    }
}

impl<T> From<Vec<Vec<T>>> for RectVec<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        let rows = v.len();
        let cols = v[0].len();

        Self(v.into_iter().flatten().collect(), rows, cols)
    }
}

impl<T> From<RectVec<T>> for Vec<T> {
    fn from(val: RectVec<T>) -> Self {
        val.0
    }
}

impl<T> IntoIterator for RectVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> std::fmt::Debug for RectVec<T>
    where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RectVec {{")?;
        for row in self.iter() {
            let mut first = true;
            for col in row {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }

                write!(f, "{:?}", col)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "}}")
    }
}

impl<T> std::fmt::Display for RectVec<T>
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter() {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut rect = RectVec::<usize>::new(2, 3);
        assert_eq!(rect.rows(), 2);
        assert_eq!(rect.cols(), 3);

        assert_eq!(rect.get(0, 0), Some(&0));
        assert_eq!(rect.row(0), &[0, 0, 0]);

        rect.set(0, 1, 1);
        assert_eq!(rect.get(0, 1), Some(&1));
        assert_eq!(rect[(0, 1)], 1);
    }

    #[test]
    fn test_from_2d_vec() {
        let rect = RectVec::from(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);

        assert_eq!(rect.rows(), 2);
        assert_eq!(rect.cols(), 3);

        assert_eq!(rect.get(0, 0), Some(&1));
        assert_eq!(rect.row(0), &[1, 2, 3]);
        assert_eq!(rect.row(1), &[4, 5, 6]);

        assert_eq!(rect.iter().collect::<Vec<_>>(), vec![&[1, 2, 3], &[4, 5, 6]]);
    }
}