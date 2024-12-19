use salsa::Event;

#[salsa::db]
#[derive(Default, Clone)]
pub struct Db {
    storage: salsa::Storage<Self>,
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