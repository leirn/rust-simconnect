/// Main Simconnect component

use tokio::task;

pub struct Simconnect {
    isConnected: bool,
    isRunning: bool,
}

impl Simconnect {

    /// Create a new Simconnect component
    pub fn new() -> Simconnect {
        Simconnect {
            isConnected: False,
            isRunning: False,
        }
    }

    /// Initiate connection with FS
    async pub fn connect(&self) -> Result<bool> {
        tokio::join!(
            run(self.run())
        );
    }

    /// Close connection with FS
    async pub fn close(&mut self) -> Result<bool> {
        self.isRunning = False;
    }

    async fn run(&self) -> Result<bool> {
        // Call dispatch ?
        while(self.isRunning) {
            tokio::time::sleep(Duration::from_millisecs(2)).await;
        }
    }
}