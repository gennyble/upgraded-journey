use std::{sync::Arc, time::Duration};

use smol::Timer;

struct Cat {
    name: &'static str,
}

impl Cat {
    fn new(name: &'static str) -> Self {
        Self { name }
    }

    fn noise(&self) -> &'static str {
        "mrrrow"
    }
}

impl Drop for Cat {
    fn drop(&mut self) {
        println!("dropped {}", self.name);
    }
}

async fn make_noise(id: &'static str, arcanimal: Arc<Cat>) {
    println!("[{}] {} says: {}", id, arcanimal.name, arcanimal.noise());
}

async fn make_noise_later(waitfor: Duration, id: &'static str, arcanimal: Arc<Cat>) {
    Timer::after(waitfor).await;

    make_noise(id, arcanimal).await;
}

fn main() {
    let mut arcanimal = Arc::new(Cat::new("levi"));

    let task1 = smol::spawn(make_noise("task1", arcanimal.clone()));
    let task2 = smol::spawn(make_noise_later(
        Duration::from_secs(2),
        "task2",
        arcanimal.clone(),
    ));

    // levi would be dropped here if they were not in an Arc
    arcanimal = Arc::new(Cat::new("genevive"));

    let task3 = smol::spawn(make_noise("task3", arcanimal.clone()));
    let task4 = smol::spawn(make_noise_later(
        Duration::from_secs(2),
        "task4",
        arcanimal.clone(),
    ));

    smol::block_on(async {
        task1.await;
        task2.await;
        task3.await;
        task4.await;
    })
}
