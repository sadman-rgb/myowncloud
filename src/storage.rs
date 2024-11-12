use serde::{Serialize, Deserialize};
use ic_cdk::storage;

#[derive(Serialize, Deserialize)]
struct FileData {
    owner: String,
    content: Vec<u8>,
}

pub fn initialize() {
    // Inisialisasi penyimpanan
    ic_cdk::storage::stable_save((Vec::<FileData>::new(),)).expect("Initialization failed");
}

pub fn upload_file(owner: String, content: Vec<u8>) {
    let file = FileData { owner, content };
    let mut files = storage::stable_restore::<Vec<FileData>>().expect("Restore failed");
    files.push(file);
    storage::stable_save((files,)).expect("Save failed");
}

pub fn download_file(owner: String) -> Option<Vec<u8>> {
    let files = storage::stable_restore::<Vec<FileData>>().expect("Restore failed");
    files.iter().find(|&f| f.owner == owner).map(|f| f.content.clone())
}
