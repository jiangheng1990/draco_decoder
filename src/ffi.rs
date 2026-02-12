use crate::DracoDecodeConfig;
#[cfg(feature = "perf")]
use std::time::Instant;

#[cxx::bridge]
mod cpp {

    // C++ structs exposed to Rust
    struct MeshAttribute {
        dim: u32,
        data_type: u32,
        offset: u32,
        length: u32,
        unique_id: u32,
    }

    struct MeshConfig {
        vertex_count: u32,
        index_count: u32,
        index_length: u32,
        attributes: Vec<MeshAttribute>,
    }

    unsafe extern "C++" {
        include!("draco_decoder/include/decoder_api.h");

        pub fn decode_point_cloud(data: &[u8]) -> Vec<u8>;

        pub unsafe fn decode_mesh_direct_write(
            data: *const u8,
            data_len: usize,
            out_ptr: *mut u8,
            out_len: usize,
        ) -> usize;

        pub unsafe fn debug_mesh_buffer_len(data: *const u8, data_len: usize) -> usize;

        // Cache API
        pub fn cache_mesh(data: &[u8]) -> u64;

        pub fn release_mesh_cache(handle: u64);

        // Get config from cache
        pub unsafe fn get_mesh_config(handle: u64, config: &mut MeshConfig) -> bool;

        // Decode to pre-allocated buffer
        pub unsafe fn decode_mesh_to_buffer(handle: u64, out_ptr: *mut u8, out_len: usize)
        -> usize;
    }
}

#[allow(dead_code)]
pub fn decode_point_cloud_native(data: &[u8]) -> Vec<u8> {
    cpp::decode_point_cloud(data)
}

pub fn decode_mesh_native(data: &[u8], config: &DracoDecodeConfig) -> Option<Vec<u8>> {
    #[cfg(feature = "perf")]
    let start = Instant::now();
    let mut out_buf = vec![0u8; config.estimate_buffer_size()];
    let written = unsafe {
        cpp::decode_mesh_direct_write(
            data.as_ptr(),
            data.len(),
            out_buf.as_mut_ptr(),
            out_buf.len(),
        )
    };
    if written == 0 || written > out_buf.len() {
        return None;
    }
    out_buf.truncate(written);
    #[cfg(feature = "perf")]
    println!("decode_mesh_native took {:?}", start.elapsed());

    Some(out_buf)
}

#[allow(dead_code)]
pub fn debug_estimate_draco_buffer_len(data: &[u8]) -> usize {
    unsafe { cpp::debug_mesh_buffer_len(data.as_ptr(), data.len()) }
}

// ---------- Cache API Wrapper ----------
pub struct MeshCache {
    handle: u64,
}

impl MeshCache {
    pub fn new(data: &[u8]) -> Option<Self> {
        let handle = cpp::cache_mesh(data);
        if handle == 0 {
            None
        } else {
            Some(MeshCache { handle })
        }
    }

    /// Decode to pre-allocated buffer, returns number of bytes written
    pub fn decode_to_buffer(&self, buffer: &mut [u8]) -> Option<usize> {
        let written =
            unsafe { cpp::decode_mesh_to_buffer(self.handle, buffer.as_mut_ptr(), buffer.len()) };
        if written == 0 { None } else { Some(written) }
    }

    pub fn get_config(&self) -> Option<crate::DracoDecodeConfig> {
        let mut cpp_config = cpp::MeshConfig {
            vertex_count: 0,
            index_count: 0,
            index_length: 0,
            attributes: Vec::new(),
        };

        if unsafe { cpp::get_mesh_config(self.handle, &mut cpp_config) } {
            Some(Self::convert_config(cpp_config))
        } else {
            None
        }
    }

    fn convert_config(cpp_config: cpp::MeshConfig) -> crate::DracoDecodeConfig {
        let mut config =
            crate::DracoDecodeConfig::new(cpp_config.vertex_count, cpp_config.index_count);

        for attr in cpp_config.attributes {
            let data_type = match attr.data_type {
                0 => crate::AttributeDataType::Int8,
                1 => crate::AttributeDataType::UInt8,
                2 => crate::AttributeDataType::Int16,
                3 => crate::AttributeDataType::UInt16,
                4 => crate::AttributeDataType::Int32,
                5 => crate::AttributeDataType::UInt32,
                6 => crate::AttributeDataType::Float32,
                _ => crate::AttributeDataType::UInt8,
            };

            // Add attribute using the public method
            config.add_attribute(attr.dim, data_type);
        }

        config
    }
}

impl Drop for MeshCache {
    fn drop(&mut self) {
        cpp::release_mesh_cache(self.handle);
    }
}

// ---------- One-shot Decode API (with config) ----------
// Use MeshDecodeResult from utils

pub fn decode_mesh_with_config(data: &[u8]) -> Option<crate::MeshDecodeResult> {
    // Step 1: Create cache and get config
    let cache = MeshCache::new(data)?;
    let config = cache.get_config()?;

    // Step 2: Pre-allocate buffer based on config
    let buffer_size = config.estimate_buffer_size();
    let mut buffer = vec![0u8; buffer_size];

    // Step 3: Decode to pre-allocated buffer
    let written = cache.decode_to_buffer(&mut buffer)?;

    // Step 4: Trim buffer to actual size and return
    buffer.truncate(written);

    Some(crate::MeshDecodeResult {
        data: buffer,
        config,
    })
}
