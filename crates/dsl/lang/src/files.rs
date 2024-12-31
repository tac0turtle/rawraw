use dashmap::DashMap;

#[derive(Default)]
pub struct FileSources {
    files: DashMap<String, String>, // TODO intern
}

#[comemo::track]
impl FileSources {
    pub fn get(&self, filename: &str) -> Option<String> {
        self.files.get(filename).map(|it| it.to_string())
    }
}

impl FileSources {
    pub fn update(&self, filename: &str, text: &str) {
        self.files.insert(filename.to_string(), text.to_string());
    }
}
