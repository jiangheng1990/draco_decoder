/// Data types for mesh attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeDataType {
    /// Signed 8-bit integer
    Int8,
    /// Unsigned 8-bit integer
    UInt8,
    /// Signed 16-bit integer
    Int16,
    /// Unsigned 16-bit integer
    UInt16,
    /// Signed 32-bit integer
    Int32,
    /// Unsigned 32-bit integer
    UInt32,
    /// 32-bit floating point
    Float32,
}

impl AttributeDataType {
    /// Returns the size in bytes of this data type.
    pub fn size_in_bytes(&self) -> usize {
        match self {
            AttributeDataType::Int8 | AttributeDataType::UInt8 => 1,
            AttributeDataType::Int16 | AttributeDataType::UInt16 => 2,
            AttributeDataType::Int32 | AttributeDataType::UInt32 | AttributeDataType::Float32 => 4,
        }
    }
}

/// Describes a single attribute in a decoded mesh.
///
/// An attribute represents per-vertex data such as positions, normals, or texture coordinates.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MeshAttribute {
    dim: u32,
    data_type: AttributeDataType,
    offset: u32,
    lenght: u32,
}

impl MeshAttribute {
    /// Creates a new mesh attribute.
    ///
    /// # Arguments
    ///
    /// * `dim` - Number of components per vertex (e.g., 3 for XYZ positions)
    /// * `data_type` - The data type of each component
    /// * `offset` - Byte offset in the decoded buffer where this attribute starts
    /// * `lenght` - Total byte length of this attribute data
    pub fn new(dim: u32, data_type: AttributeDataType, offset: u32, lenght: u32) -> Self {
        Self {
            dim,
            data_type,
            offset,
            lenght,
        }
    }

    /// Returns the byte offset of this attribute in the decoded buffer.
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// Returns the total byte length of this attribute data.
    pub fn lenght(&self) -> u32 {
        self.lenght
    }

    /// Returns the data type of this attribute.
    pub fn data_type(&self) -> AttributeDataType {
        self.data_type
    }

    /// Returns the number of components per vertex.
    pub fn dim(&self) -> u32 {
        self.dim
    }
}

/// Configuration and metadata for a decoded Draco mesh.
///
/// This struct contains all the information needed to interpret the decoded
/// mesh buffer, including vertex count, index count, and attribute layouts.
#[derive(Debug, PartialEq, Eq)]
pub struct DracoDecodeConfig {
    vertex_count: u32,
    index_count: u32,
    index_length: u32,
    buffer_size: usize,
    attributes: Vec<MeshAttribute>,
}

impl DracoDecodeConfig {
    /// Creates a new config with a pre-computed buffer size.
    ///
    /// Used internally when decoding from C++ FFI.
    pub(crate) fn new(vertex_count: u32, index_count: u32, buffer_size: usize) -> Self {
        let index_length = if index_count <= u16::MAX as u32 {
            index_count as usize * 2
        } else {
            index_count as usize * 4
        } as u32;

        Self {
            vertex_count,
            index_count,
            index_length,
            buffer_size,
            attributes: Vec::new(),
        }
    }

    /// Returns the total byte length of the index data.
    pub fn index_length(&self) -> u32 {
        self.index_length
    }

    /// Adds an attribute with specified offset and length.
    ///
    /// Used internally when receiving attribute data from C++ FFI.
    pub(crate) fn add_attribute(
        &mut self,
        dim: u32,
        data_type: AttributeDataType,
        offset: u32,
        length: u32,
    ) {
        let attribute = MeshAttribute {
            dim,
            data_type,
            offset,
            lenght: length,
        };
        self.attributes.push(attribute);
    }

    /// Returns the attribute at the given index, if it exists.
    pub fn get_attribute(&self, index: usize) -> Option<&MeshAttribute> {
        self.attributes.get(index)
    }

    /// Returns a vector of all attributes.
    pub fn attributes(&self) -> Vec<MeshAttribute> {
        self.attributes.clone()
    }

    /// Returns the number of vertices in the mesh.
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    /// Returns the number of indices in the mesh.
    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    /// Returns the total buffer size required for the decoded mesh.
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl DracoDecodeConfig {
    /// Returns the estimated buffer size for the decoded mesh.
    ///
    /// This is an alias for `buffer_size()`.
    pub fn estimate_buffer_size(&self) -> usize {
        self.buffer_size
    }
}

/// Typed values for a decoded mesh attribute.
#[derive(Debug)]
pub enum AttributeValues {
    /// Signed 8-bit integer values
    Int8(Vec<i8>),
    /// Unsigned 8-bit integer values
    UInt8(Vec<u8>),
    /// Signed 16-bit integer values
    Int16(Vec<i16>),
    /// Unsigned 16-bit integer values
    UInt16(Vec<u16>),
    /// Signed 32-bit integer values
    Int32(Vec<i32>),
    /// Unsigned 32-bit integer values
    UInt32(Vec<u32>),
    /// 32-bit floating point values
    Float32(Vec<f32>),
}

/// Result of decoding a Draco mesh.
///
/// Contains the decoded mesh buffer and metadata describing its layout.
#[derive(Debug)]
pub struct MeshDecodeResult {
    /// The decoded mesh buffer containing indices and attribute data.
    pub data: Vec<u8>,
    /// Metadata describing the mesh structure and attribute layouts.
    pub config: DracoDecodeConfig,
}
