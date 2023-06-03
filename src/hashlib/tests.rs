use crate::{hash_types::HashType, errors::HashTypeError};

#[test]
fn test_all_hash_types() {

    // Test Sha1 & Sha2 lengths
    let sha1_length = 160;
    let sha224_length = 224;
    let sha256_length = 256;
    let sha384_length = 384;
    let sha512_length = 512;

    // MD5 Length
    let md5_length = 128;
    

    assert_eq!(HashType::SHA1,   HashType::try_from(sha1_length).unwrap());
    assert_eq!(HashType::SHA224, HashType::try_from(sha224_length).unwrap());
    assert_eq!(HashType::SHA256, HashType::try_from(sha256_length).unwrap());
    assert_eq!(HashType::SHA384, HashType::try_from(sha384_length).unwrap());
    assert_eq!(HashType::SHA512, HashType::try_from(sha512_length).unwrap());
    assert_eq!(HashType::MD5,    HashType::try_from(md5_length).unwrap());
}

#[test]
fn hash_type_error_test() {
    let invalid_length = 20;
    let test = HashType::try_from(invalid_length).map_err(|e| e);
    let expected = Err(HashTypeError::InvalidHashSize(20));

    assert_eq!(expected, test);
}

#[test]
fn normal_hash() {
    use sha2::{Sha256, Digest};
    
    let plain_text = "sara";

    // "sara" hashed as Sha256
    let hashed = "3a6d64c24cf80b69ccda37650406467e8266667b50cfd0b984beb3651b129ed7";

    assert_eq!(hashed, format!("{:x}", Sha256::digest(plain_text.as_bytes())))
}