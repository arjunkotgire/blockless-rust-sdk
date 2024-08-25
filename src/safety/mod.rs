use tokio::sync::Mutex;
use std::sync::Arc;

pub struct SafeData {
    data: Arc<Mutex<i32>>,
}

impl SafeData {
    pub fn new() -> Self {
        SafeData {
            data: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn update_data(&self, value: i32) {
        let mut data = self.data.lock().await;
        *data = value;
    }

    pub async fn read_data(&self) -> i32 {
        let data = self.data.lock().await;
        *data
    }
}
