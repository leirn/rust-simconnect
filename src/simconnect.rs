use std::{sync::Arc, time::Duration};
/// Main Simconnect component
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Simconnect {
    pub inner: Arc<Mutex<InternalSimconnect>>,
}

impl Simconnect {
    /// Create a new Simconnect component
    pub fn new() -> Simconnect {
        Simconnect {
            inner: Arc::new(Mutex::new(InternalSimconnect::new())),
        }
    }
    pub async fn get_running_status(&self) -> bool {
        let inner = self.inner.lock().await;
        (*inner).is_running
    }

    /// Initiate connection with FS
    pub async fn connect(&self) -> tokio::task::JoinHandle<()> {
        let inner = Arc::clone(&self.inner);
        inner.lock().await.is_running = true;
        tokio::spawn(async move {
            let mut tmp = inner.lock().await;
            tmp.run().await;
        })
    }

    /// Close connection with FS
    pub async fn close(&mut self) {
        self.inner.lock().await.is_running = false;
    }
}

pub struct InternalSimconnect {
    pub is_running: bool,
}
impl InternalSimconnect {
    /// Create a new Simconnect component
    pub fn new() -> InternalSimconnect {
        InternalSimconnect { is_running: false }
    }

    pub async fn run(&mut self) {
        self.is_running = true;
        // Call dispatch ?
        while self.is_running {
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    }
}
