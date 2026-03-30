use plog_auth::PasswordHasher;

fn main() {
    let password = "admin123";
    let hash = PasswordHasher::hash(password).unwrap();

    println!("Password: {}", password);
    println!("Hash: {}", hash);
}
