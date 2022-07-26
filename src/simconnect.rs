use std::{sync::Arc, time::Duration};
/// Main Simconnect component
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Simconnect {
    pub inner: Arc<Mutex<InternalSimconnect>>,
}

impl Default for Simconnect {
    fn default() -> Self {
        Self::new()
    }
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
        unsafe {
            let h_sim_connect: std::os::windows::raw::HANDLE = 0 as std::os::windows::raw::HANDLE;
            let lib = libloading::Library::new("C:\\MSFS SDK\\SimConnect SDK\\lib\\SimConnect.dll")
                .unwrap();
            let sim_open: libloading::Symbol<
                unsafe extern "C" fn(
                    std::os::windows::raw::HANDLE,
                    &[u8],
                    u32,
                    u32,
                    u32,
                    u32,
                ) -> u32,
            > = lib.get(b"SimConnect_Open").unwrap();
            let _hr = sim_open(h_sim_connect, b"Test\0", 0, 0, 0, 0);
        }

        self.is_running = true;
        // Call dispatch ?
        while self.is_running {
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    }
}
impl Default for InternalSimconnect {
    fn default() -> Self {
        Self::new()
    }
}
