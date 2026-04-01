use anyhow::Result;
use object_store::ObjectStoreExt;
use object_store::{local::LocalFileSystem, ObjectStore, path::Path as ObjectPath, PutPayload};
use std::sync::Arc;
use blake3::Hasher;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bytes::BytesMut;

pub struct StorageService {
    store: Arc<LocalFileSystem>,
    base_path: String,
}

impl StorageService {
    pub fn new(base_path: String) -> Self {
        let store = Arc::new(LocalFileSystem::new_with_prefix(&base_path).expect("Failed to create local store"));
        Self { store, base_path }
    }

    pub async fn upload_file(&self, username: &str, filename: &str, mut file: File) -> Result<(String, String)> {
        let user_dir = format!("users/{}", username);
        let full_path = format!("{}/{}", user_dir, filename);
        let object_path = ObjectPath::from(full_path.as_str());

        let mut hasher = Hasher::new();
        let mut size_bytes: i64 = 0;
        let mut buffer = BytesMut::with_capacity(8192 * 4);  // efficient chunking

        // Stream read + hash + collect for put
        let mut data = Vec::new();
        loop {
            let mut chunk = vec![0u8; 8192];
            let n = file.read(&mut chunk).await?;
            if n == 0 { break; }
            let chunk = &chunk[..n];
            hasher.update(chunk);
            data.extend_from_slice(chunk);
            size_bytes += n as i64;
        }

        let checksum = hasher.finalize().to_hex().to_string();
        let payload = PutPayload::from(BytesMut::from(data.as_slice()).freeze());

        self.store.put(&object_path, payload).await?;

        println!("✅ File written to HDD: {}", full_path);
        Ok((full_path, checksum))
    }

    pub fn get_full_path(&self, relative: &str) -> String {
        format!("{}/{}", self.base_path, relative)
    }
}