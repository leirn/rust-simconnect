extern crate simconnect;
use simconnect::simconnect::Simconnect;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut simconnect = Simconnect::new();
    let _connexion = simconnect.connect().await;
    simconnect.close().await;
}

#[cfg(test)]
mod tests {
    #[actix_rt::test]
    async fn init_simconnect() {
        let simconnect: simconnect::simconnect::Simconnect =
            simconnect::simconnect::Simconnect::new();
        let running = simconnect.get_running_status().await;
        assert_eq!(running, false);
    }

    #[actix_rt::test]
    async fn start_simconnect() {
        let simconnect: simconnect::simconnect::Simconnect =
            simconnect::simconnect::Simconnect::new();
        simconnect.connect().await;
        let running = simconnect.get_running_status().await;
        assert_eq!(running, true);
    }

    #[actix_rt::test]
    async fn start_and_stop_simconnect() {
        let mut simconnect: simconnect::simconnect::Simconnect =
            simconnect::simconnect::Simconnect::new();
        simconnect.connect().await;
        simconnect.close().await;
        let running = simconnect.get_running_status().await;
        assert_eq!(running, false);
    }
}
