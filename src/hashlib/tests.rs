use crate::{hash_types::HashType, errors::HashTypeError};

#[test]
fn hash_type_test() {
    let sha256_bit_length = 256;
    
    assert_eq!(HashType::SHA256, HashType::try_from(sha256_bit_length).unwrap());
}

#[test]
fn hash_type_error_test() {
    let invalid_length = 20;
    let test = HashType::try_from(invalid_length).map_err(|e| e);
    let expected = Err(HashTypeError::InvalidHashSize(20));

    assert_eq!(expected, test);
}