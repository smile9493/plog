use plog_auth::PasswordHasher;

fn main() {
    let password = "admin123";
    match PasswordHasher::hash(password) {
        Ok(hash) => {
            println!("Password: {}", password);
            println!("Hash: {}", hash);
        }
        Err(e) => {
            eprintln!("Failed to hash password: {}", e);
            std::process::exit(1);
        }
    }
}
