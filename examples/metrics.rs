use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const MAX_THREAD: usize = 2;
const MAX_REQUEST: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::<i64>::new();
    for i in 0..MAX_THREAD {
        task_worker(i, metrics.clone())?;
    }
    for i in 0..MAX_REQUEST {
        request_worker(i, metrics.clone())?; // Metrics {data: Arc::clone(&metrics.data)}
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        let snapshot = metrics.snapshot();
        println!("{:?}", snapshot);
    }
}

fn task_worker(idx: usize, metrics: Metrics<i64>) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do some term suff
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            metrics.inc(format!("call.thread.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(idx: usize, metrics: Metrics<i64>) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do some term suff
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            metrics.inc(format!("req.page.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
