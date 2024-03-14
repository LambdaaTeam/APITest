use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Vec<u8> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    hash.to_string().into_bytes()
}

pub fn verify_password(hash: &Vec<u8>, password: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = std::str::from_utf8(hash).unwrap();
    let password_hash = PasswordHash::new(parsed_hash).unwrap();

    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}
