pub mod triangle;
pub mod trianglemesh;
pub mod material;

pub use trianglemesh::TriangleMesh;
pub use triangle::{Triangle, TriangleIndex};
pub use material::Material;

/// Position in 3D space.
type Position = glam::Vec3;
/// Normal vector in 3D space. Should be normalized, but this is not enforced.
type Normal = glam::Vec3;
/// Texture coordinate in 2D space.
type TexCoord = glam::Vec2;
