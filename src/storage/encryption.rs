use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use argon2::Argon2;
use rand::RngCore;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

pub struct EncryptionEngine {
    cipher: Aes256Gcm,
}

impl EncryptionEngine {
    pub fn from_machine_key() -> Result<Self> {
        let machine_id = get_machine_id()?;
        let salt = derive_stable_salt(&machine_id);
        let key = derive_key(machine_id.as_bytes(), &salt)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .context("Failed to create cipher")?;
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher.encrypt(nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {e}"))?;

        let mut output = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        output.extend_from_slice(&nonce_bytes);
        output.extend_from_slice(&ciphertext);
        Ok(output)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < NONCE_LEN {
            anyhow::bail!("Data too short to contain nonce");
        }
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);

        self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {e}"))
    }
}

fn derive_key(password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
    let mut key = vec![0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(password, salt, &mut key)
        .map_err(|e| anyhow::anyhow!("Key derivation failed: {e}"))?;
    Ok(key)
}

fn get_machine_id() -> Result<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .context("Failed to get machine ID")?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("IOPlatformUUID") {
                if let Some(uuid) = line.split('"').nth(3) {
                    return Ok(uuid.to_string());
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(id) = std::fs::read_to_string("/etc/machine-id") {
            return Ok(id.trim().to_string());
        }
        if let Ok(id) = std::fs::read_to_string("/var/lib/dbus/machine-id") {
            return Ok(id.trim().to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("wmic")
            .args(["csproduct", "get", "UUID"])
            .output()
            .context("Failed to get machine ID")?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().nth(1) {
            return Ok(line.trim().to_string());
        }
    }

    Ok(hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "compscan-default".into()))
}

fn derive_stable_salt(machine_id: &str) -> Vec<u8> {
    let mut salt = vec![0u8; SALT_LEN];
    let id_bytes = machine_id.as_bytes();
    for (i, byte) in salt.iter_mut().enumerate() {
        *byte = id_bytes[i % id_bytes.len()] ^ 0x5A;
    }
    salt
}
