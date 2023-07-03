/// Data defining a vector of three floats (f32). Contains the position of the
/// vertex.
///
/// **Data info GPU:**
/// - size: 16 bytes (4 bytes padding)
/// - alignment: 16 bytes
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vec3Data {
    /// The position of the vertex.
    pub data: [f32; 3],
    _padding0: u32,
}

impl Vec3Data {
    /// Creates a new vector data.
    pub fn new(data: [f32; 3]) -> Self {
        Self { data, _padding0: 0 }
    }
}

/// Data defining a triangle, by storing the indices of the vertices, the normal
/// of the triangle, and the material of the triangle.
///
/// **Data info GPU:**
/// - size: 20 bytes (no padding)
/// - alignment: 4 bytes
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct TriangleData {
    /// Index to first vertex of the triangle.
    pub v0_index: u32,
    /// Index to second vertex of the triangle.
    pub v1_index: u32,
    /// Index to third vertex of the triangle.
    pub v2_index: u32,
    /// Index to normal of the triangle.
    pub normal_index: u32,
    /// Index to material of the triangle.
    pub material_index: u32,
}

impl TriangleData {
    /// Creates a new triangle data.
    pub fn new(
        v0_index: u32,
        v1_index: u32,
        v2_index: u32,
        normal_index: u32,
        material_index: u32,
    ) -> Self {
        Self {
            v0_index,
            v1_index,
            v2_index,
            normal_index,
            material_index,
        }
    }
}

/// Data defining an object of multiple triangles and a bounding box. Contains
/// the bounding box min and max positions, the index to the first triangle and the
/// number of triangles.
///
/// **Data info GPU:**
/// - size: 48 bytes (16 bytes padding)
/// - alignment: 16 bytes
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ObjectData {
    pub aabb_min: [f32; 3],
    _padding0: u32,
    pub aabb_max: [f32; 3],
    pub triangle_start_index: u32,
    pub triangle_count: u32,
    _padding1: [u32; 3],
}

impl ObjectData {
    /// Creates a new object data.
    pub fn new(
        aabb_min: [f32; 3],
        aabb_max: [f32; 3],
        triangle_start_index: u32,
        triangle_count: u32,
    ) -> Self {
        Self {
            aabb_min,
            _padding0: 0,
            aabb_max,
            triangle_start_index,
            triangle_count,
            _padding1: [0; 3],
        }
    }

    pub fn without_bounding_box(triangle_start_index: u32, triangle_count: u32) -> Self {
        Self::new([0.0; 3], [0.0; 3], triangle_start_index, triangle_count)
    }
}

/// Data defining a material
///
/// **Data info GPU:**
/// - size 64 bytes (16 bytes padding)
/// - alignment: 16 bytes
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct MaterialData {
    pub ambient_color: [f32; 3],
    _padding0: u32,
    pub diffuse_color: [f32; 3],
    _padding1: u32,
    pub specular_color: [f32; 3],
    _padding2: u32,
    pub emission_color: [f32; 3],
    pub specular_highlight: f32,
    pub index_of_refraction: f32,
    pub transparency: f32,
    _padding3: [u32; 2],
}

impl MaterialData {
    /// Creates a new material data with the given data.
    pub fn new(
        ambient_color: [f32; 3],
        diffuse_color: [f32; 3],
        specular_color: [f32; 3],
        emission_color: [f32; 3],
        specular_highlight: f32,
        index_of_refraction: f32,
        transparency: f32,
    ) -> Self {
        Self {
            ambient_color,
            _padding0: 0,
            diffuse_color,
            _padding1: 0,
            specular_color,
            _padding2: 0,
            emission_color,
            specular_highlight,
            index_of_refraction,
            transparency,
            _padding3: [0; 2],
        }
    }
}

/// Data defining a camera.
///
/// **Data info GPU:**
/// - size: 144 bytes (1 bytes padding)
/// - alignment: 16 bytes
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraData {
    pub inv_projection: [f32; 16],
    pub inv_view: [f32; 16],
    pub position: [f32; 3],
    _padding: u32,
}

impl CameraData {
    /// Creates a new camera data.
    pub fn new(inv_projection: [f32; 16], inv_view: [f32; 16], position: [f32; 3]) -> Self {
        Self {
            inv_projection,
            inv_view,
            position,
            _padding: 0,
        }
    }
}

/// Data defining the render settings.
///
/// **Data info GPU:**
/// - size: 16 bytes (no padding)
/// - alignment: 4 bytes
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RenderData {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub bounces: u32,
}

impl RenderData {
    /// Creates a new render data.
    pub fn new(width: u32, height: u32, samples: u32, bounces: u32) -> Self {
        Self {
            width,
            height,
            samples,
            bounces,
        }
    }
}

/// Gpu data as bytes. This is used to send the data to the GPU.
pub struct GPUBytes<'data> {
    pub vertex_positions: &'data [u8],
    pub vertex_normals: &'data [u8],
    pub triangles: &'data [u8],
    pub objects: &'data [u8],
    pub materials: &'data [u8],
    pub camera: &'data [u8],
    pub render: &'data [u8],
}

/// Collection of all data that is sent to the GPU.
pub struct GPUData {
    pub vertex_positions: Vec<Vec3Data>,
    pub vertex_normals: Vec<Vec3Data>,
    pub triangles: Vec<TriangleData>,
    pub objects: Vec<ObjectData>,
    pub materials: Vec<MaterialData>,
    pub camera: CameraData,
    pub render_data: RenderData,
}

impl GPUData {
    /// Creates uninitialized GPU data.
    #[inline]
    pub fn new() -> Self {
        Self {
            vertex_positions: Vec::new(),
            vertex_normals: Vec::new(),
            triangles: Vec::new(),
            objects: Vec::new(),
            materials: Vec::new(),
            camera: CameraData::new([0.0; 16], [0.0; 16], [0.0; 3]),
            render_data: RenderData::new(0, 0, 0, 0),
        }
    }

    /// Sets the camera.
    #[inline]
    pub fn set_camera(&mut self, camera: CameraData) {
        self.camera = camera;
    }

    /// Sets the render settings.
    #[inline]
    pub fn set_render(&mut self, render_data: RenderData) {
        self.render_data = render_data;
    }

    /// Sets the vertex positions.
    #[inline]
    pub fn set_vertex_positions(&mut self, vertex_positions: Vec<Vec3Data>) {
        self.vertex_positions = vertex_positions;
    }

    /// Sets the vertex normals.
    #[inline]
    pub fn set_vertex_normals(&mut self, vertex_normals: Vec<Vec3Data>) {
        self.vertex_normals = vertex_normals;
    }

    /// Sets the triangles.
    #[inline]
    pub fn set_triangles(&mut self, triangles: Vec<TriangleData>) {
        self.triangles = triangles;
    }

    /// Sets the objects.
    #[inline]
    pub fn set_objects(&mut self, objects: Vec<ObjectData>) {
        self.objects = objects;
    }

    /// Sets the materials.
    #[inline]
    pub fn set_materials(&mut self, materials: Vec<MaterialData>) {
        self.materials = materials;
    }

    /// Converts the data to bytes. This is required to send the data to the
    /// GPU. `GPUBytes` is a struct containing slices to the data.
    #[inline]
    pub fn to_bytes(&self) -> GPUBytes {
        GPUBytes {
            vertex_normals: bytemuck::cast_slice(&self.vertex_normals),
            vertex_positions: bytemuck::cast_slice(&self.vertex_positions),
            triangles: bytemuck::cast_slice(&self.triangles),
            objects: bytemuck::cast_slice(&self.objects),
            materials: bytemuck::cast_slice(&self.materials),
            camera: bytemuck::bytes_of(&self.camera),
            render: bytemuck::bytes_of(&self.render_data),
        }
    }
}

impl Default for GPUData {
    #[inline]
    /// Same as `new` method.
    fn default() -> Self {
        Self::new()
    }
}
