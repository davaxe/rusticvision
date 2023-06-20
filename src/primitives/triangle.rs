use super::{material::Material, Normal, Position};

/// Single indices of a triangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriangleIndex {
    vertex_indices: (usize, usize, usize),
    normal_index: usize,
    material_index: usize,
}

impl TriangleIndex {
    /// Creates a new triangle from the given vertex indices, normal index and
    /// material index.
    ///
    /// ### Arguments
    /// * `vertex_indices` - The vertex indices of the triangle.
    /// * `normal_index` - The normal index of the triangle.
    /// * `material_index` - The material index of the triangle.
    ///
    /// ### Return value
    /// The newly created triangle.
    pub fn new(
        vertex_indices: (usize, usize, usize),
        normal_index: usize,
        material_index: usize,
    ) -> Self {
        Self {
            vertex_indices,
            normal_index,
            material_index,
        }
    }

    /// Returns the vertex indices of the triangle.
    ///
    /// ### Return value
    /// (v1, v2, v3)
    pub fn vertex_indices(&self) -> (usize, usize, usize) {
        self.vertex_indices
    }

    /// Returns the material index of the triangle.
    pub fn material_index(&self) -> usize {
        self.material_index
    }

    /// Returns the normal index of the triangle.
    pub fn normal_index(&self) -> usize {
        self.normal_index
    }

    /// Return vertex indices, normal index and material index as a tuple in the
    /// order specified below.
    ///
    /// ### Return value
    /// (v1, v2, v3, n, m)
    pub fn indices(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.vertex_indices.0,
            self.vertex_indices.1,
            self.vertex_indices.2,
            self.normal_index,
            self.material_index,
        )
    }
}

impl From<&Vec<usize>> for TriangleIndex {
    /// Creates a new triangle from a vec of values. The vec must contain
    /// at least 4 values. If it contains more than 5 values, only the first 5
    /// will be used.
    ///
    /// ### Panics
    /// Panics if the vec is shorter than 4 elements.
    fn from(values: &Vec<usize>) -> Self {
        Self::from(values.as_slice())
    }
}

impl<const N: usize> From<[usize; N]> for TriangleIndex {
    /// Creates a new triangle from a array of values. The array must contain
    /// at least 4 values. If it contains more than 5 values, only the first 5
    /// will be used.
    ///
    /// ### Panics
    /// Panics if the array is shorter than 4 elements.
    fn from(value: [usize; N]) -> Self {
        Self::from(&value[..])
    }
}

impl From<&[usize]> for TriangleIndex {
    /// Creates a new triangle from a slice of values. The slice must contain
    /// at least 4 values. If it contains more than 5 values, only the first 5
    /// will be used.
    ///
    /// ### Panics
    /// Panics if the slice is shorter than 4 elements.
    fn from(values: &[usize]) -> Self {
        if values.len() >= 5 {
            Self::new((values[0], values[1], values[2]), values[3], values[4])
        } else if values.len() == 4 {
            Self::new((values[0], values[1], values[2]), values[3], 0)
        } else {
            panic!("Invalid triangle definition: {:?}", values);
        }
    }
}

impl Default for TriangleIndex {
    /// Creates a new triangle with all indices set to 0.
    fn default() -> Self {
        Self::new((0, 0, 0), 0, 0)
    }
}

/// Struct of all data needed to define a triangle. This includes the vertices,
/// the normal and the material. Data is stored as references.
pub struct Triangle<'global> {
    vertices: (&'global Position, &'global Position, &'global Position),
    normal: &'global Normal,
    material: &'global Material,
}

impl<'global> Triangle<'global> {
    pub fn new(
        vertices: (&'global Position, &'global Position, &'global Position),
        normal: &'global Normal,
        material: &'global Material,
    ) -> Self {
        Self {
            vertices,
            normal,
            material,
        }
    }
}
