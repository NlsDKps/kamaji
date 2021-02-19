use log::error;
/// Hash password with SHA-512
pub fn hash_password<'a>(password: &'a str) -> Option<String> {
    let hasher = match botan::HashFunction::new("SHA-512") {
        Ok(hasher) => hasher,
        Err(e) => {
            error!("Could not generate SHA-3 hasher: {:#?}", e);
            return None
        }
    };
    match hasher.update(password.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not update hasher: {:#?}", e);
            return None
        }
    };
    let digest = match hasher.finish() {
        Ok(digest) => digest,
        Err(e) => {
            error!("Could not finish hashing: {:#?}", e);
            return None
        }
    };
    match botan::hex_encode(&digest) {
        Ok(hash) => Some(hash),
        Err(e) => {
            error!("Could not hex encode digest: {:?}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        env_logger::init();
        let test_string = "test";
        let expected_string = "EE26B0DD4AF7E749AA1A8EE3C10AE9923F618980772E473F8819A5D4940E0DB27AC185F8A0E1D5F84F88BC887FD67B143732C304CC5FA9AD8E6F57F50028A8FF";
        assert_eq!(hash_password(test_string).unwrap(), expected_string);
    }
}
