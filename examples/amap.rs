use anyhow::Result;
use concurrency::AmapMetric as Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const MAX_THREAD: usize = 2;
const MAX_REQUEST: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new(&[
        "call.thread.0",
        "call.thread.1",
        "req.page.0",
        "req.page.1",
        "req.page.2",
        "req.page.3",
    ]);
    for i in 0..MAX_THREAD {
        task_worker(i, metrics.clone())?;
    }
    for i in 0..MAX_REQUEST {
        request_worker(i, metrics.clone())?; // Metrics {data: Arc::clone(&metrics.data)}
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
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

fn request_worker(idx: usize, metrics: Metrics) -> Result<()> {
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
