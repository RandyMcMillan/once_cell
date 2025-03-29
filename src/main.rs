use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

static SHARED_RESOURCE: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn initialize_resource() -> String {
    // Simulate an asynchronous operation (using thread::sleep for simplicity)
    thread::sleep(Duration::from_millis(100));
    "Initialized Resource".to_string()
}

fn get_resource() -> String {
    let mut guard = SHARED_RESOURCE
        .get_or_init(|| Mutex::new(None))
        .lock()
        .unwrap();

    if guard.is_none() {
        *guard = Some(initialize_resource());
    }

    guard.clone().unwrap()
}

fn main() {
    let resource1 = get_resource();
    let resource2 = get_resource();

    println!("Resource 1: {}", resource1);
    println!("Resource 2: {}", resource2);

    assert_eq!(resource1, resource2);
}
