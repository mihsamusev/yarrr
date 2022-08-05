pub mod body;
pub mod camera;
pub mod image;
pub mod linalg;
pub mod material;
pub mod ray;

pub mod prelude {
    pub use crate::body::*;
    pub use crate::camera::*;
    pub use crate::image::*;
    pub use crate::linalg::*;
    pub use crate::material::*;
    pub use crate::ray::*;
}
