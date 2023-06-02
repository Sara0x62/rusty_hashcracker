use crate::errors::HashTypeError;

#[repr(usize)]
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]    // - For testing purposes
pub enum HashType {
    // Hash type = bits
    // SHA-1 - deprecated
    SHA1 = 160,
    
    // SHA-2 versions
    SHA224 = 224,
    SHA256 = 256,
    SHA384 = 384,
    SHA512 = 512,

    // MD5 version
    MD5 = 128,
}

impl TryFrom<usize> for HashType {
    type Error = HashTypeError;
    fn try_from(t: usize) -> Result<Self, HashTypeError> {
        match t {
            160 => Ok(HashType::SHA1),

            224 => Ok(HashType::SHA224),
            256 => Ok(HashType::SHA256),
            384 => Ok(HashType::SHA384),
            512 => Ok(HashType::SHA512),

            120 => Ok(HashType::MD5),

            _ => Err(HashTypeError::InvalidHashSize(t))
        }
    }
}
