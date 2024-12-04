/// Calculates the greatest common divisor of two numbers.
/// 
/// # Examples
/// ```
/// use aoc2023::helpers::nums::gcd;
/// 
/// assert_eq!(gcd(2, 3), 1);
/// assert_eq!(gcd(3, 4), 1);
/// assert_eq!(gcd(4, 2), 2);
/// ```
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + PartialOrd + std::ops::Rem<Output = T> + std::ops::Sub<Output = T> + Default,
{
    let (mut min, mut max) = if a < b {
        (a, b)
    } else {
        (b, a)
    };

    loop {
        let remainder = max % min;
        if remainder == T::default() {
            return min;
        }

        (min, max) = (remainder, min);    
    }
}

/// Calculates the least common multiple of two numbers.
/// 
/// # Examples
/// ```
/// use aoc2023::helpers::nums::lcm;
/// 
/// assert_eq!(lcm(2, 3), 6);
/// assert_eq!(lcm(3, 4), 12);
/// assert_eq!(lcm(4, 2), 4);
/// ```
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + PartialOrd + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + std::ops::Rem<Output = T> + std::ops::Sub<Output = T> + Default,
{
    a * b / gcd(a, b)
}

pub fn abs<T>(val: T) -> T
where
    T: Default + PartialOrd + std::ops::Neg<Output = T> + Copy,
{
    if val < T::default() {
        -val
    } else {
        val
    }
}

/// A complex number composed of its real and imaginary components.
/// 
/// # Examples
/// ```
/// use aoc2023::helpers::nums::Complex;
/// 
/// let c1 = Complex::new(1, 2);
/// let c2 = Complex::new(3, 4);
/// 
/// assert_eq!(c1 + c2, Complex::new(4, 6));
/// assert_eq!(c1 - c2, Complex::new(-2, -2));
/// ```
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self{re, im}
    }
}

impl<T> Complex<T>
    where T: Default + PartialOrd + std::ops::Neg<Output = T> + Copy
{
    pub fn abs(&self) -> Self {
        Self {
            re: if self.re < T::default() { -self.re } else { self.re },
            im: if self.im < T::default() { -self.im } else { self.im },
        }
    }
}

impl<T> std::ops::Add<Self> for Complex<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T> std::ops::Sub<Self> for Complex<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{ re: self.re - rhs.re, im: self.im - rhs.im }
    }
}

impl<T> std::ops::Mul<Self> for Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T> std::ops::Mul<T> for Complex<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self{ re: self.re * rhs, im: self.im * rhs }
    }
}

impl<T> std::ops::Div<T> for Complex<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self{ re: self.re / rhs, im: self.im / rhs }
    }
}

impl<T> std::ops::Neg for Complex<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self{ re: -self.re, im: -self.im }
    }
}

impl<T> std::ops::Div<Self> for Complex<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.re * rhs.re + rhs.im * rhs.im;
        Self{
            re: (self.re * rhs.re + self.im * rhs.im) / denominator,
            im: (self.im * rhs.re - self.re * rhs.im) / denominator,
        }
    }
}

impl<T> std::ops::Rem<T> for Complex<T>
where
    T: std::ops::Rem<Output = T> + Copy,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self{
            re: self.re % rhs,
            im: self.im % rhs,
        }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from((re, im): (T, T)) -> Self {
        Self{ re, im }
    }
}

impl<T> From<Complex<T>> for (T, T) {
    fn from(val: Complex<T>) -> Self {
        (val.re, val.im)
    }
}

impl<T> std::fmt::Display for Complex<T>
    where T: std::fmt::Display + Copy + Default + PartialOrd
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.re != T::default() {
            write!(f, "{}", self.re)?;
        }

        if self.im != T::default() {
            if self.im < T::default() {
                write!(f, "{}i", self.im)?;
            } else {
                write!(f, "+{}i", self.im)?;
            }
        }

        Ok(())
    }
}

impl<T> std::fmt::Debug for Complex<T>
    where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Complex")
            .field("real", &self.re)
            .field("imaginary", &self.im)
            .finish()
    }
}