pub mod body;
pub mod image;
pub mod linalg;
pub mod ray;

pub mod prelude {
    pub use crate::body::*;
    pub use crate::image::*;
    pub use crate::linalg::*;
    pub use crate::ray::*;
}
