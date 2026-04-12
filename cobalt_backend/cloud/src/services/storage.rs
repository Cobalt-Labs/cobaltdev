use anyhow::Result;
use object_store::{local::LocalFileSystem, ObjectStoreExt, path::Path as ObjectPath, PutPayload};
use std::sync::Arc;
use blake3::Hasher;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bytes::Bytes;

pub struct StorageService {
    store: Arc<LocalFileSystem>,
    base_path: String,
}

impl StorageService {
    pub fn new(base_path: String) -> Self {
        let store = Arc::new(LocalFileSystem::new_with_prefix(&base_path)
            .expect("Failed to create local store"));
        Self { store, base_path }
    }

    pub async fn upload_file(
        &self,
        username: &str,
        filename: &str,
        mut file: File,
    ) -> Result<(String, String, i64)> {
        let user_dir = format!("users/{}", username);
        let full_path = format!("{}/{}", user_dir, filename);
        let object_path = ObjectPath::from(full_path.as_str());

        let mut hasher = Hasher::new();
        let mut size_bytes: i64 = 0;
        let mut data = Vec::new();

        let mut buffer = vec![0u8; 8192];

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            let chunk = &buffer[..n];
            hasher.update(chunk);
            data.extend_from_slice(chunk);
            size_bytes += n as i64;
        }

        let checksum = hasher.finalize().to_hex().to_string();
        let payload = PutPayload::from(Bytes::from(data));

        self.store.put(&object_path, payload).await?;

        println!(
            "File written: {} ({} bytes, checksum: {})",
            full_path, size_bytes, checksum
        );

        Ok((full_path, checksum, size_bytes))
    }
}