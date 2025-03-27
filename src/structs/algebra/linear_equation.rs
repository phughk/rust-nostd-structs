use crate::structs::AsType;
use core::ops::*;

/// A linear equation in a 2d space
pub struct LinearEquation<T> {
    expr: LinearExpression<T>,
}

enum LinearExpression<T> {
    /// a, b for ax + b
    Slope { a: T, b: T },
    /// x for f(x) = y (i.e. any y is f(x))
    Vertical(T),
}

impl<T> LinearEquation<T> {
    /// Create a linear equation from the provided parameters
    pub fn new(slope: T, y_intercept: T) -> Self {
        Self {
            expr: LinearExpression::Slope {
                a: slope,
                b: y_intercept,
            },
        }
    }

    /// Get the y value for the position x
    pub fn y(&self, x: T) -> Option<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        match self.expr {
            LinearExpression::Slope { a, b } => Some(a * x + b),
            LinearExpression::Vertical(_) => None,
        }
    }

    /// Get the x value for the position y
    pub fn x(&self, y: T) -> T
    where
        T: Copy + Sub<Output = T> + Div<Output = T> + PartialEq + AsType<f32>,
    {
        match self.expr {
            LinearExpression::Slope { a, b } => {
                if a == T::from_type(0.0) {
                    b
                } else {
                    (y - b) / a
                }
            }
            LinearExpression::Vertical(x) => x,
        }
    }

    /// Create a linear equation from 2 points
    pub fn from_2_points(p1: (T, T), p2: (T, T)) -> Self
    where
        T: Copy
            + Sub<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + PartialEq
            + AsType<f32>,
    {
        let x_diff = p2.0 - p1.0;
        if x_diff == T::from_type(0.0) {
            return Self {
                expr: LinearExpression::Vertical(p1.0),
            };
        }
        let slope = (p2.1 - p1.1) / x_diff;
        let y_intercept = p1.1 - slope * p1.0;
        Self {
            expr: LinearExpression::Slope {
                a: slope,
                b: y_intercept,
            },
        }
    }

    /// Create a linear equation orthogonal to the current one at the provided point
    pub fn orthogonal_at_point(&self, x: T, y: T) -> Self
    where
        T: Copy
            + Sub<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Neg<Output = T>
            + PartialEq
            + AsType<f32>,
    {
        match self.expr {
            LinearExpression::Slope { a, .. } => {
                let slope = -a;
                if slope == T::from_type(0.0) {
                    Self {
                        expr: LinearExpression::Vertical(x),
                    }
                } else {
                    let y_intercept = y - slope * x;
                    Self {
                        expr: LinearExpression::Slope {
                            a: slope,
                            b: y_intercept,
                        },
                    }
                }
            }
            LinearExpression::Vertical(_) => {
                let slope = T::from_type(0.0);
                let y_intercept = y;
                Self {
                    expr: LinearExpression::Slope {
                        a: slope,
                        b: y_intercept,
                    },
                }
            }
        }
    }

    /// Project the point (x, y) onto the line
    pub fn project_onto(&self, x: T, y: T) -> (T, T)
    where
        T: Copy
            + Sub<Output = T>
            + Div<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Neg<Output = T>
            + PartialEq
            + AsType<f32>,
    {
        match self.expr {
            LinearExpression::Slope { a, b } => {
                let other = self.orthogonal_at_point(x, y);
                match other.expr {
                    LinearExpression::Slope {
                        a: other_a,
                        b: other_b,
                    } => {
                        let x = (other_b - b) / (a - other_a);
                        let y = self.y(x).unwrap();
                        (x, y)
                    }
                    LinearExpression::Vertical(x) => {
                        let y = self.y(x).unwrap();
                        (x, y)
                    }
                }
            }
            LinearExpression::Vertical(x) => (x, y),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::structs::algebra::linear_equation::LinearEquation;

    #[test]
    fn test_project_onto() {
        let eq = LinearEquation::from_2_points((1.0, 1.0), (2.0, 2.0));
        let (x, y) = eq.project_onto(1.0, 2.0);
        assert_eq!(x, 1.5);
        assert_eq!(y, 1.5);

        let horizontal = LinearEquation::from_2_points((1.0, 1.0), (2.0, 1.0));
        let (x, y) = horizontal.project_onto(1.0, 2.0);
        assert_eq!(x, 1.0);
        assert_eq!(y, 1.0);

        let vertical = LinearEquation::from_2_points((1.0, 1.0), (1.0, 2.0));
        let (x, y) = vertical.project_onto(2.0, 1.0);
        assert_eq!(x, 1.0);
        assert_eq!(y, 1.0);
    }
}
