#![forbid(unsafe_code)]

use pasts::prelude::*;
use pasts::CvarExec;

async fn timer_future(duration: std::time::Duration) {
    pasts::spawn_blocking(move || std::thread::sleep(duration)).await
}

fn main() {
    static EXECUTOR: CvarExec = CvarExec::new();
    let ret = EXECUTOR.block_on(async {
        println!("Waiting 2 seconds…");
        timer_future(std::time::Duration::new(2, 0)).await;
        println!("Waited 2 seconds.");
        "Complete!"
    });
    println!("Future returned: \"{}\"", ret);
}
