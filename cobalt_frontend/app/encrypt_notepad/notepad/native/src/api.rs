use flutter_rust_bridge::frb;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Once;

use std::sync::OnceLock;
use sha2::{Sha256, Digest};

static DERIVED_KEY: OnceLock<[u8; 32]> = OnceLock::new();

fn get_encryption_key() -> &'static [u8; 32] {
    DERIVED_KEY.get_or_init(|| {
        let mut hasher = Sha256::new();
        hasher.update(b"NOTEPAD_SECRET_KEY_2025");
        hasher.update(std::env::var("USER").unwrap_or_default().as_bytes());
        let result = hasher.finalize();
        result.into()
    })
}

fn calculate_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

static ENCRYPTION_KEY: &[u8] = b"NOTEPAD_SECRET_KEY_2025";
static INIT: Once = Once::new();

fn initialize() {
    let dir = notes_dir();
    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Failed to create notes directory: {}", e);
        } else {
            println!("Notes directory created at: {:?}", dir);
        }
    }
}

fn notes_dir() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_default();
    dir.push("notes");
    dir
}

fn ensure_directory_exists() -> io::Result<PathBuf> {
    INIT.call_once(|| {
        initialize();
    });
    let dir = notes_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

fn xor_encrypt_decrypt(data: &[u8]) -> Vec<u8> {
    data.iter()
        .zip(ENCRYPTION_KEY.iter().cycle())
        .map(|(&b, &k)| b ^ k)
        .collect()
}

fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();
    let data = data.trim();
    
    if data.len() % 4 != 0 {
        return Err("Invalid base64 length".to_string());
    }
    
    for chunk in data.as_bytes().chunks(4) {
        let mut buf = [0u8; 4];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = match ALPHABET.iter().position(|&x| x == byte) {
                Some(pos) => pos as u8,
                None if byte == b'=' => 0,
                None => return Err("Invalid base64 character".to_string()),
            };
        }
        
        let combined = ((buf[0] as u32) << 18)
            | ((buf[1] as u32) << 12)
            | ((buf[2] as u32) << 6)
            | (buf[3] as u32);
        
        result.push((combined >> 16) as u8);
        if chunk[2] != b'=' {
            result.push((combined >> 8) as u8);
        }
        if chunk[3] != b'=' {
            result.push(combined as u8);
        }
    }
    
    Ok(result)
}

#[frb]
pub fn save_note_to_disk(title: String, content: String) -> bool {
    let sanitized = title.chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { '_' })
        .collect::<String>()
        .trim()
        .to_string();
    
    if sanitized.is_empty() {
        eprintln!("[Rust] Invalid title after sanitization");
        return false;
    }
    
    let base_path = if cfg!(target_os = "android") {
        std::env::var("ANDROID_DATA")
            .unwrap_or_else(|_| "/data/data".to_string())
    } else {
        std::env::var("HOME")
            .unwrap_or_else(|_| "/tmp".to_string())
    };
    
    let notes_dir = if cfg!(target_os = "android") {
        format!("{}/files/notes", base_path)
    } else {
        format!("{}/Documents/encrypted_notes", base_path)
    };
    
    let path = PathBuf::from(format!("{}/{}.txt", notes_dir, sanitized));
    
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("[Rust] Failed to create folder {:?}: {}", parent, e);
            return false;
        }
    }
    
    let encrypted = xor_encrypt_decrypt(content.as_bytes());
    match File::create(&path).and_then(|mut f| f.write_all(&encrypted)) {
        Ok(_) => {
            println!("[Rust] Saved note to {:?}", path);
            true
        }
        Err(e) => {
            eprintln!("[Rust] File write failed: {}", e);
            false
        }
    }
}

#[frb]
pub fn load_note_from_disk(title: String) -> String {
    let base_path = if cfg!(target_os = "android") {
        std::env::var("ANDROID_DATA")
            .unwrap_or_else(|_| "/data/data".to_string())
    } else {
        std::env::var("HOME")
            .unwrap_or_else(|_| "/tmp".to_string())
    };
    
    let notes_dir = if cfg!(target_os = "android") {
        format!("{}/files/notes", base_path)
    } else {
        format!("{}/Documents/encrypted_notes", base_path)
    };
    
    let path = PathBuf::from(format!("{}/{}.txt", notes_dir, title));
    
    match fs::read(&path) {
        Ok(encrypted_bytes) => {
            let decrypted_bytes = xor_encrypt_decrypt(&encrypted_bytes);
            match String::from_utf8(decrypted_bytes) {
                Ok(content) => {
                    println!("[Rust] Loaded note from {:?}", path);
                    content
                }
                Err(e) => {
                    eprintln!("[Rust] Failed to decode UTF-8: {}", e);
                    String::new()
                }
            }
        }
        Err(e) => {
            eprintln!("[Rust] Failed to read file {:?}: {}", path, e);
            String::new()
        }
    }
}

#[frb]
pub async fn list_note_titles() -> String {
    let dir = match ensure_directory_exists() {
        Ok(dir) => dir,
        Err(_) => return String::new(),
    };
    
    let mut titles = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if let Some(title) = name.strip_suffix(".txt") {
                    titles.push(title.to_string());
                }
            }
        }
    }
    
    titles.sort();
    titles.join(";")
}

#[frb]
pub fn delete_note_from_disk(title: String) {
    let title_str = title;
    
    let dir = match ensure_directory_exists() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to access notes directory for deletion: {}", e);
            return;
        }
    };
    
    let mut path = dir;
    path.push(format!("{}.txt", title_str));
    
    if let Err(e) = fs::remove_file(&path) {
        eprintln!("Failed to delete note {}: {}", title_str, e);
    } else {
        println!("Note deleted successfully: {}", title_str);
    }
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        
        let triple = (b0 << 16) | (b1 << 8) | b2;
        
        result.push(ALPHABET[(triple >> 18) as usize] as char);
        result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(ALPHABET[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

#[frb]
pub fn encrypt_text(text: String) -> String {
    let encrypted_bytes = xor_encrypt_decrypt(text.as_bytes());
    let encoded = base64_encode(&encrypted_bytes);
    format!("🔒 {}", encoded)
}

#[frb]
pub fn decrypt_text(encrypted_text: String) -> String {
    let clean_text = if encrypted_text.starts_with("🔒 ") {
        &encrypted_text[4..]
    } else {
        &encrypted_text
    };
    
    match base64_decode(clean_text) {
        Ok(decoded_bytes) => {
            let decrypted_bytes = xor_encrypt_decrypt(&decoded_bytes);
            match String::from_utf8(decrypted_bytes) {
                Ok(decrypted_text) => decrypted_text,
                Err(e) => {
                    eprintln!("[Rust] Failed to decode decrypted UTF-8: {}", e);
                    String::new()
                }
            }
        }
        Err(e) => {
            eprintln!("[Rust] Failed to decode base64: {}", e);
            encrypted_text
        }
    }
}

#[frb]
pub fn get_notes_directory() -> Result<String, anyhow::Error> {
    let base_path = if cfg!(target_os = "android") {
        std::env::var("ANDROID_DATA")
            .map(|data| format!("{}/data/com.example.notepad/files", data))
            .unwrap_or_else(|_| "/data/data/com.example.notepad/files".to_string())
    } else {
        std::env::var("HOME")
            .map(|home| format!("{}/Documents/encrypted_notes", home))
            .unwrap_or_else(|_| "/tmp/encrypted_notes".to_string())
    };
    
    let notes_dir = PathBuf::from(format!("{}/notes", base_path));
    
    if !notes_dir.exists() {
        std::fs::create_dir_all(&notes_dir)?;
    }
    
    Ok(notes_dir.to_string_lossy().to_string())
}