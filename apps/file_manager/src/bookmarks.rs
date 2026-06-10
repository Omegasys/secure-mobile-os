use std::path::PathBuf;

#[derive(Clone)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
}

pub struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
        }
    }

    pub fn add_bookmark(
        &mut self,
        name: String,
        path: PathBuf,
    ) {
        self.bookmarks.push(
            Bookmark {
                name,
                path,
            }
        );
    }

    pub fn remove_bookmark(
        &mut self,
        name: &str,
    ) {
        self.bookmarks
            .retain(|b| b.name != name);
    }

    pub fn get_bookmark(
        &self,
        name: &str,
    ) -> Option<&Bookmark> {
        self.bookmarks
            .iter()
            .find(|b| b.name == name)
    }

    pub fn all(
        &self,
    ) -> &[Bookmark] {
        &self.bookmarks
    }
}
