use pyo3::prelude::*;
use pyo3::types::PyByteArray;

/// XXTEA encryption implementation.
/// In Rust, this can be called with slices.
/// In Python, `data` should be a `bytearray` to allow in-place modification.
#[pyfunction]
#[pyo3(name = "xxtea_encrypt")]
pub fn py_xxtea_encrypt(data: &Bound<'_, PyByteArray>, key: &[u8]) -> PyResult<()> {
    // Safety: We ensure we don't hold references to the buffer longer than necessary.
    // PyByteArray::as_bytes_mut provides a mutable slice to the underlying Python buffer.
    let data_slice = unsafe { data.as_bytes_mut() };
    xxtea_encrypt(data_slice, key);
    Ok(())
}

/// Pure Rust implementation of XXTEA encryption.
pub fn xxtea_encrypt(data: &mut [u8], key: &[u8]) {
    const N: usize = 32;

    // Ensure we have enough data (32 * 4 = 128 bytes)
    if data.len() < N * 4 {
        return;
    }

    // Convert data to u32 vector (Little Endian)
    let mut v: Vec<u32> = data[..N * 4]
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    // Convert key to u32 vector (Little Endian)
    // Key is expected to be 16 bytes (4 u32s)
    let mut k_vec: Vec<u32> = key
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    // If key is shorter than 16 bytes, pad with 0
    while k_vec.len() < 4 {
        k_vec.push(0);
    }
    // We only need the first 4 u32s
    let k = &k_vec[..4];

    let mut sum: u32 = 0;
    let mut a3 = v[N - 1];
    let mut a4;

    let mut a7 = 6;
    while a7 > 0 {
        a7 -= 1;
        sum = sum.wrapping_add(0x9E3779B9);
        let a5 = (sum >> 2) & 3;

        for p in 0..(N - 1) {
            a4 = v[p + 1];

            let term1 = (a3 >> 5) ^ (a4 << 2);
            let term2 = (a4 >> 3) ^ (a3 << 4);
            let term3 = sum ^ a4;
            let term4 = k[(p & 3) ^ a5 as usize] ^ a3;

            let val = term1.wrapping_add(term2) ^ term3.wrapping_add(term4);
            v[p] = v[p].wrapping_add(val);
            a3 = v[p];
        }

        // Final step for the round
        a4 = v[0];
        let p = N - 1;

        let term1 = (a3 >> 5) ^ (a4 << 2);
        let term2 = (a4 >> 3) ^ (a3 << 4);
        let term3 = sum ^ a4;
        let term4 = k[(p & 3) ^ a5 as usize] ^ a3;

        let val = term1.wrapping_add(term2) ^ term3.wrapping_add(term4);
        v[p] = v[p].wrapping_add(val);
        a3 = v[p];
    }

    // Write back to data (Little Endian)
    for (i, val) in v.iter().enumerate() {
        let bytes = val.to_le_bytes();
        data[i * 4] = bytes[0];
        data[i * 4 + 1] = bytes[1];
        data[i * 4 + 2] = bytes[2];
        data[i * 4 + 3] = bytes[3];
    }
}
