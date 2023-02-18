use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::thread;
use std::time::Duration;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use tokio::join;
use tokio::time::sleep;

struct Container {
    goal: i32,
    counter: Arc<Mutex<i32>>,
}

impl Future for Container {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let ref_self = self.counter.clone();
        if *ref_self.lock().unwrap() == self.goal * 2 {
            return Poll::Ready(format!("The doubled number is - {}", self.goal * 2).to_string());
        } else {
            println!("Counter status at - {}", *ref_self.lock().unwrap());
            thread::sleep(Duration::from_millis(25));
            *ref_self.lock().unwrap() += 1;
            cx.waker().clone().wake();
            Poll::Pending
        }
    }
}

fn format_my_num(num: i32) -> impl Future<Output = String> {
    Container {
        counter: Arc::new(Mutex::new(0)),
        goal: num,
    }
}

async fn format_async(num: i32) -> String {
    for i in 0..num {
        println!("counting wow - {i}");
        sleep(Duration::from_millis(800)).await;
    }
    format!("OMG - {}", num).to_string()
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let my_string = format_my_num(60);
    let new_string = format_async(32);
    let (old, new) = join!(my_string, new_string);

    println!("{old} - {new}");
    Ok(())
}
