use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug)]
#[allow(dead_code)]
struct Mssage {
    idx: usize,
    value: usize,
}

const NUM_PRODUCERS: usize = 4;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for idx in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || {
            producer(idx, tx).unwrap();
        });
    }
    drop(tx); // close the channel (tx

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit\n");
        "secret"
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    println!("secret: {}\n", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Mssage>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        let msg = Mssage::new(idx, value);
        tx.send(msg)?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit\n", idx);
            break;
        }
        thread::sleep(Duration::from_millis(sleep_time));
    }
    Ok(())
}

impl Mssage {
    fn new(idx: usize, value: usize) -> Self {
        Mssage { idx, value }
    }
}
