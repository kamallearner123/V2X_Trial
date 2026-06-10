use libc::{c_int, size_t, uint8_t};

// Rust representation of the C BSMData struct
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BSMData {
    pub vehicle_id: u32,
    pub latitude: i32,
    pub longitude: i32,
    pub speed: u16,
    pub heading: i16,
}

// Link to the C shared library `libasn1.so`
extern "C" {
    // int asn1_encode(const BSMData* in_data, uint8_t* out_buffer, size_t* out_len);
    pub fn asn1_encode(
        in_data: *const BSMData,
        out_buffer: *mut uint8_t,
        out_len: *mut size_t,
    ) -> c_int;

    // int asn1_decode(const uint8_t* in_buffer, size_t in_len, BSMData* out_data);
    pub fn asn1_decode(
        in_buffer: *const uint8_t,
        in_len: size_t,
        out_data: *mut BSMData,
    ) -> c_int;
}
