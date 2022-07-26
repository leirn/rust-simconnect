extern crate simconnect;
use std::time::Duration;
use simconnect::simconnect::Simconnect;

#[tokio::main]
async fn main() {
    let mut simconnect = Simconnect::new();
    let _connexion = simconnect.connect().await.unwrap();
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
    async fn start_and_stop_simconnect() {
        let mut simconnect: simconnect::simconnect::Simconnect =
            simconnect::simconnect::Simconnect::new();
        //let running = simconnect.get_running_status().await;
        //assert_eq!(running, false);
        simconnect.connect().await.unwrap();
        let running = simconnect.get_running_status().await;
        assert_eq!(running, true);
        simconnect.close().await;
        let running = simconnect.get_running_status().await;
        assert_eq!(running, false);
    }
}
