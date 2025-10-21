use std::fmt;
use std::ops::{Add, Sub};

/// A 2D vector with generic components of the same type.
/// 
/// Type parameter:
/// - `T`: component type (e.g., i32, f64). Must support the traits required
///   by the implemented methods/traits (see impl blocks).
#[derive(Debug, Clone, PartialEq)]
pub struct Direction<T> {
    pub x: T,
    pub y: T,
}

impl<T> Direction<T> {
    /// Construct a new `Direction<T>` from `(x, y)`.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Return a nicely formatted string version (e.g., "(x, y)").
    /// Kept separate so we can reuse it in both `print()` and `Display`.
    /// 
    /// Requires that `T` implements `Display`.
    pub fn format(&self) -> String
    where
        T: fmt::Display,
    {
        format!("({}, {})", self.x, self.y)
    }

    /// Print the direction in a nice format.
    /// 
    /// Requires that `T` implements `Display`.
    pub fn print(&self)
    where
        T: fmt::Display,
    {
        println!("{}", self.format());
    }
}

impl<T> Direction<T>
where
    T: Copy + Add<Output = T>,
{
    /// Add two directions component-wise and return the result.
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Implement `Display` so `println!("{}", dir)` uses our nice format.
impl<T> fmt::Display for Direction<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the same formatting used by `print()`
        write!(f, "{}", self.format())
    }
}

/// Enable `+` for `Direction<T>`, adding component-wise.
impl<T> Add for Direction<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Enable `-` for `Direction<T>`, subtracting component-wise.
impl<T> Sub for Direction<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_and_print() {
        let d = Direction::new(3, 4);
        assert_eq!(d.to_string(), "(3, 4)");
        // d.print(); // (manual check)
    }

    #[test]
    fn method_add() {
        let a = Direction::new(1, 2);
        let b = Direction::new(10, 20);
        let c = Direction::add(&a, &b);
        assert_eq!(c, Direction::new(11, 22));
    }

    #[test]
    fn operator_add() {
        let a = Direction::new(1.5, -2.0);
        let b = Direction::new(0.5, 4.0);
        let c = a + b;
        assert_eq!(c, Direction::new(2.0, 2.0));
    }

    #[test]
    fn operator_sub() {
        let a = Direction::new(7, 5);
        let b = Direction::new(2, 9);
        let c = a - b;
        assert_eq!(c, Direction::new(5, -4));
    }
}
