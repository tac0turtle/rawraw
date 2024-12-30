use dashmap::DashMap;
use salsa::Event;

#[salsa::db]
#[derive(Default, Clone)]
pub struct Db {
    storage: salsa::Storage<Self>,
    document_map: DashMap<String, FileSource>,
}

#[salsa::input]
pub struct FileSource {
    #[return_ref]
    pub text: String,
}

#[salsa::db]
impl salsa::Database for Db {
    fn salsa_event(&self, event: &dyn Fn() -> Event) {}
}

#[salsa::db]
pub trait DatabaseExt: salsa::Database {
    fn file_source(&self, url: &str) -> Option<FileSource>;
    fn add_file_source(&self, url: String, source: FileSource);
}

#[salsa::db]
impl DatabaseExt for Db {
    fn file_source(&self, url: &str) -> Option<FileSource> {
        self.document_map.get(url).map(|it| it.clone())
    }

    fn add_file_source(&self, url: String, source: FileSource) {
        self.document_map.insert(url, source);
    }
}