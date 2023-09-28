use crate::*;
use std::*;

pub fn sha256hex_of_path(
    hasher: &mut sha2::Sha256,
    text_file_path_s: &mut String,
    pat: &std::path::Path,
) -> Result<String, CustomErr> {
    let direntp_utf8 = pat.to_str().ok_or("Path not valid UTF-8")?;
    let retval = sha256_of_bytes(hasher, direntp_utf8.as_bytes())?;
    *text_file_path_s = direntp_utf8.to_owned();
    Ok(bytes2hex(&retval))
}
pub fn sha256hex_of_str(hasher: &mut sha2::Sha256, instr: &str) -> Result<String, CustomErr> {
    let retval = sha256_of_bytes(hasher, instr.as_bytes())?;
    Ok(bytes2hex(&retval))
}
pub fn sha256_of_bytes(hasher: &mut sha2::Sha256, bytes: &[u8]) -> Result<[u8; 32], CustomErr> {
    use sha2::Digest;
    hasher.update(bytes);
    let hash_bytes = hasher.finalize_reset();
    //let hash_bytes = hasher.finalize_boxed_reset();
    //use base64::{engine::general_purpose, Engine as _};
    //return Ok(general_purpose::STANDARD_NO_PAD.encode(hash_bytes));
    let retval: [u8; 32] = hash_bytes.as_slice().try_into()?;
    Ok(retval)
}
