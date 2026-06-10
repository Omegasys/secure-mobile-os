mod browser;
mod search;
mod permissions;

use browser::FileBrowser;
use search::SearchEngine;
use permissions::PermissionManager;

pub struct FileManagerApp {
    pub browser: FileBrowser,
    pub search: SearchEngine,
    pub permissions: PermissionManager,
}

impl FileManagerApp {
    pub fn new() -> Self {
        Self {
            browser: FileBrowser::new(),
            search: SearchEngine::new(),
            permissions: PermissionManager::new(),
        }
    }

    pub fn start(&self) {
        println!("File Manager started");
    }
}

fn main() {
    let app = FileManagerApp::new();
    app.start();
}
