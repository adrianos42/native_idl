use super::session::Session;
use std::sync::Arc;
use tower_lsp::{Client};

pub struct Server {
    pub(crate) client: Client,
    pub(crate) session: Arc<Session>,
}

impl Server {
    pub fn new(client: Client) -> Result<Self, i64> {
        let session = Arc::new(Session::new(client.clone())?);
        Ok(Server { client, session })
    }
}