#[repr(usize)]
#[derive(Debug)]
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

impl From<usize> for HashType {
    fn from(t: usize) -> Self {
        match t {
            160 => HashType::SHA1,

            224 => HashType::SHA224,
            256 => HashType::SHA256,
            384 => HashType::SHA384,
            512 => HashType::SHA512,

            120 => HashType::MD5,

            _ => panic!("Invalid Hash length: {}", t)
        }
    }
}
