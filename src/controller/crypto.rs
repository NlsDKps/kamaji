use crypto::sha3::Sha3;
use crypto::digest::Digest;

/// Hash password with SHA3-512
pub fn hash_password<'a>(password: &'a str) -> String {
    let mut hasher = Sha3::sha3_512();
    hasher.input_str(password);
    hasher.result_str()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let test_string = "test";
        let expected_string = "9ece086e9bac491fac5c1d1046ca11d737b92a2b2ebd93f005d7b710110c0a678288166e7fbe796883a4f2e9b3ca9f484f521d0ce464345cc1aec96779149c14";
        assert_eq!(hash_password(test_string), expected_string);
    }
}
