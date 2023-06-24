pub mod triangle;
pub mod trianglemesh;
pub mod ray;
pub mod aabb;

pub use aabb::BoundingBox;
pub use trianglemesh::TriangleMesh;
pub use ray::{Ray, Hit};
pub use triangle::{Triangle, TriangleIndex};

/// Position in 3D space.
type Position = glam::Vec3A;
/// Normal vector in 3D space. Should be normalized, but this is not enforced.
type Normal = glam::Vec3A;
/// Texture coordinate in 2D space.
type TexCoord = glam::Vec2;
