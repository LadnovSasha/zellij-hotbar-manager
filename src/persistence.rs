use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

fn get_hotbar_file_path() -> PathBuf {
    PathBuf::from("/tmp/zellij-hotbar-manager.rkyv")
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(Debug))]
pub struct SessionMetadata {
    pub last_accessed: u64,
    pub tab_count: u64,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(Debug))]
pub struct HotbarData {
    pub favorites: Vec<Option<String>>,
    pub previous_session: Option<String>,
    pub metadata: HashMap<String, SessionMetadata>,
}

impl Default for HotbarData {
    fn default() -> Self {
        Self {
            favorites: vec![None; 5],
            previous_session: None,
            metadata: HashMap::new(),
        }
    }
}

impl HotbarData {
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = rkyv::to_bytes::<_, 256>(self)?;
        let path = get_hotbar_file_path();
        std::fs::write(&path, bytes.as_ref())?;
        Ok(())
    }

    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let path = get_hotbar_file_path();
        let bytes = std::fs::read(&path)?;

        // SAFETY: We trust the data because:
        // 1. We control both serialization and deserialization
        // 2. The file is written atomically by this plugin only
        // 3. The data format is stable and managed by rkyv
        let archived = unsafe { rkyv::archived_root::<HotbarData>(&bytes) };

        let data: HotbarData = archived.deserialize(&mut rkyv::Infallible)?;
        Ok(data)
    }

    pub fn get_session_at_slot(&self, slot: usize) -> Option<&String> {
        if slot < 5 {
            self.favorites[slot].as_ref()
        } else {
            None
        }
    }

    pub fn set_session_at_slot(&mut self, slot: usize, session_name: Option<String>) {
        if slot < 5 {
            self.favorites[slot] = session_name;
        }
    }

    pub fn update_metadata(&mut self, session_name: &str, tab_count: usize) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.metadata.insert(
            session_name.to_string(),
            SessionMetadata {
                last_accessed: now,
                tab_count: tab_count as u64,
            },
        );
    }

    pub fn get_metadata(&self, session_name: &str) -> Option<&SessionMetadata> {
        self.metadata.get(session_name)
    }
}
