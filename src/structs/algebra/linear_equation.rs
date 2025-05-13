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
    pub fn x(&self, y: T) -> Option<T>
    where
        T: Copy + Sub<Output = T> + Div<Output = T> + PartialEq + AsType<f32>,
    {
        match self.expr {
            LinearExpression::Slope { a, b } => {
                if a == T::from_type(0.0) {
                    if b == y {
                        // Any point on "x" is valid
                        Some(T::from_type(0.0))
                    } else {
                        // No point on "x" is valid
                        None
                    }
                } else {
                    Some((y - b) / a)
                }
            }
            LinearExpression::Vertical(x) => Some(x),
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
                let slope = -(T::from_type(1.0) / a);
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

    /// Translate the line by the provided x and y values
    pub fn translate(&self, x: T, y: T) -> Self
    where
        T: Mul<Output = T> + Add<Output = T> + Copy + Sub<Output = T>,
    {
        match &self.expr {
            LinearExpression::Slope { a, b } => {
                let new_b = *b + y - *a * x;
                Self {
                    expr: LinearExpression::Slope { a: *a, b: new_b },
                }
            }
            LinearExpression::Vertical(ox) => Self {
                expr: LinearExpression::Vertical(*ox + x),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::assert_float_equal_f32;
    use crate::structs::algebra::linear_equation::LinearEquation;
    use crate::structs::geom::{Line2D, Point2D};
    use proptest::prelude::*;
    use proptest::proptest;
    use std::println;

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

    #[test]
    fn test_orthogonal() {
        let p1 = Point2D::new(0.82891583, -8.229318);
        let p2 = Point2D::new(1.9410644, -16.112713);
        let direct = LinearEquation::from_2_points(p1.as_tuple(), p2.as_tuple());
        let start_x = 0.0;
        let end_x = 2.0;
        let orth1 = direct.orthogonal_at_point(p1.x, p1.y);
        let orth_line_1 = {
            let start_y = orth1.y(start_x).unwrap();
            let end_y = orth1.y(end_x).unwrap();
            Line2D::new(Point2D::new(start_x, start_y), Point2D::new(end_x, end_y))
        };
        let orth2 = direct.orthogonal_at_point(p2.x, p2.y);
        let orth_line_2 = {
            let start_y = orth2.y(start_x).unwrap();
            let end_y = orth2.y(end_x).unwrap();
            Line2D::new(Point2D::new(start_x, start_y), Point2D::new(end_x, end_y))
        };
        #[cfg(feature = "helpers")]
        {
            use crate::structs::geom::PrintDesmos;

            let direct_line = Line2D::new(p1, p2);
            println!("Direct line: {}", direct_line.to_string_desmos());
            println!("Orthogonal line 1: {}", orth_line_1.to_string_desmos());
            println!("Orthogonal line 2: {}", orth_line_2.to_string_desmos());
        }
        panic!()
    }

    #[derive(Debug)]
    struct TestInput {
        x1: f32,
        x2: f32,
        y1: f32,
        y2: f32,
        t1: f32,
        tx1: f32,
        ty1: f32,
        tx2: f32,
        ty2: f32,
    }

    fn test_input_strategy() -> BoxedStrategy<TestInput> {
        let prop_x1 = any::<i8>();
        let prop_x2 = any::<i8>();
        let prop_y1 = any::<i8>();
        let prop_y2 = any::<i8>();
        let prop_t1 = any::<i8>();
        let prop_tx1 = any::<i8>();
        let prop_ty1 = any::<i8>();
        let prop_tx2 = any::<i8>();
        let prop_ty2 = any::<i8>();
        (
            prop_x1, prop_x2, prop_y1, prop_y2, prop_t1, prop_tx1, prop_ty1, prop_tx2, prop_ty2,
        )
            .prop_map(|(x1, x2, y1, y2, t1, tx1, ty1, tx2, ty2)| {
                (
                    x1 as f32 / 10.0,
                    x2 as f32 / 10.0,
                    y1 as f32 / 10.0,
                    y2 as f32 / 10.0,
                    t1 as f32 / 10.0,
                    tx1 as f32 / 10.0,
                    ty1 as f32 / 10.0,
                    tx2 as f32 / 10.0,
                    ty2 as f32 / 10.0,
                )
            })
            .prop_filter(
                "Points on original line must be unique",
                |(x1, x2, y1, y2, _t1, _tx1, _ty1, _tx2, _ty2)| {
                    // Points on original line must be different
                    x1 != x2 || y1 != y2
                },
            )
            .prop_filter(
                "Points on orthogonal line must be unique",
                |(_x1, _x2, _y1, _y2, _t1, tx1, ty1, tx2, ty2)| {
                    // Points on orthogonal line must be different
                    tx1 != tx2 || ty1 != ty2
                },
            )
            .prop_map(|(x1, x2, y1, y2, t1, tx1, ty1, tx2, ty2)| TestInput {
                x1,
                x2,
                y1,
                y2,
                t1,
                tx1,
                ty1,
                tx2,
                ty2,
            })
            .boxed()
    }

    fn unit_test_project_line(input: TestInput) {
        // We can translate any line twice across the X-axis and the projections will be the same
        let original = LinearEquation::from_2_points((input.x1, input.y1), (input.x2, input.y2));
        let orth = {
            let (x, y) = match original.x(input.t1) {
                None => (input.t1, original.y(input.t1).unwrap()),
                Some(v) => (input.t1, v),
            };
            let orth = original.orthogonal_at_point(x, y);
            orth
        };
        // Any 2 points on orthogonal line will have the same projection
        // Generate points on orthogonal line
        let (ox1, oy1) = orth.project_onto(input.tx1, input.ty1);
        let (ox2, oy2) = orth.project_onto(input.tx2, input.ty2);
        // Generate projections onto original
        let (p1x, p1y) = original.project_onto(ox1, oy1);
        let (p2x, p2y) = original.project_onto(ox2, oy2);
        // Projected points should be identical
        assert_float_equal_f32(p1x, p2x, 0.001);
        assert_float_equal_f32(p1y, p2y, 0.001);
    }

    #[test]
    fn test_nan_project_line() {
        let input = TestInput {
            x1: -6.090175e18,
            x2: 0.0,
            y1: 3.7716197e-22,
            y2: 0.0,
            t1: 0.0,
            tx1: 0.0,
            ty1: 0.0,
            tx2: 0.0,
            ty2: 4.631469e-18,
        };
        unit_test_project_line(input);
    }

    proptest! {

        #[test]
        fn prop_test_project_line(input in test_input_strategy()) {
            unit_test_project_line(input);
        }
    }
}
