use anyhow::{anyhow, Context};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use tokio::task;

// Hashing function using argon2 algorithm
pub async fn hash(password: String) -> anyhow::Result<String> {
    // Spawn a new task to prevent any blovking
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| anyhow!(err).context("Failed to hash password"))?
            .to_string())
    })
    .await
    .context("panic in hash function")?
}

// FUnction to compare a password and a hash
pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash)
            .map_err(|err| anyhow!(err).context("password hash invalid"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("Failed to verify password")),
        }
    })
    .await
    .context("panic in verify function")?
}
