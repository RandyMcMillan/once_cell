use async_std::sync::{Arc, Mutex};
use async_std::task;

#[async_std::main]
async fn main() {
    let _say_hello_result = say_hello().await;
    let m1 = Arc::new(Mutex::new(0));
    let m2 = m1.clone();

    task::spawn(async move {
        *m2.lock().await = 1;
    })
    .await;

    assert_eq!(*m1.lock().await, 1);
    println!("{:?}", m1);
    println!("{:?}", *m1.lock().await);
    let a = async { 1u8 };
    let b = async { 2u8 };
    println!("{:?}", a.await);
    println!("{:?}", b.await);
    //assert_eq!(a.join(b).await, (1u8, 2u8));
}
async fn say_hello() {
    println!("Hello, world!");
}
