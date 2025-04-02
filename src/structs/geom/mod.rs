//! Multidimensional geometry structs that are used in multidimensional algorithms
mod line_2d;
mod misc;
mod point_2d;
mod polygon_2d;
mod shape_2d;
mod triangle_2d;

pub use line_2d::Line2D;
pub use point_2d::Point2D;
pub use polygon_2d::Polygon2D;
pub use shape_2d::Shape2D;
pub use triangle_2d::Triangle2D;

#[cfg(feature = "helpers")]
/// Pretty print for easy copy-pasting into Desmos
/// https://www.desmos.com/calculator
pub trait PrintDesmos {
    /// Print the object in a format that Desmos can understand
    fn to_string_desmos(&self) -> arrayvec::ArrayString<1024>;
}
