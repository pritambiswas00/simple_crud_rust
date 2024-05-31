use bcrypt::{ hash, verify, DEFAULT_COST };

pub fn encode_password(password: &str)->String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash:&str)->bool {
    verify(password, hash).unwrap()
}

