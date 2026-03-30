use std::sync::{Arc, Mutex, mpsc};
use std::thread;
// use crossbeam_channel::{unbounded, Sender, Receiver};

pub struct Job {
    pub id: usize,
    pub done: bool 
}

// a worker receives jobs on a channel 
pub struct Worker {
    pub id: usize,
    pub is_busy: bool,
    // (queue)for receiving jobs
    pub job_rx: Arc<Mutex<mpsc::Receiver<Job>>>,
    
    // for sending results 
    pub result_tx: mpsc::Sender<Result<String, String>>
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> mpsc::Receiver<Result<String, String>> {
        let (result_tx, result_rx) = mpsc::channel();
        // create a new worker 
        let mut worker = Worker {
            is_busy: false,
            id,
            job_rx: rx,
            result_tx
        };

        thread::spawn(move || { 
            worker.run();
        });

        result_rx
    } 

    pub fn run(&mut self) {
        // wait for job
        while let Ok(mut job) = self.job_rx.lock().unwrap().recv() {
            self.is_busy = true;
            println!("worker #{:?} received job #{:?}", self.id, job.id);
            
            thread::sleep_ms(2000);
            
            // finish job 
            job.done = true;

            self.result_tx.send(Ok(format!("worker #{:?} finished job #{:?}", self.id, job.id))).ok();
            
            self.is_busy = false;
        } 
    }
}

fn main() {
    // println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
        let (job_tx, job_rx) = mpsc::channel::<Job>();
        let job_rx_ref = Arc::new(Mutex::new(job_rx));
        
        let mut workers = Vec::<mpsc::Receiver<Result<String, String>>>::new();
        // fill worker pool 
        for id in 0..10 {
            workers.push(Worker::new(id, job_rx_ref.clone()));
        }

        // create some jobs
        // send each job through job_tx(some worker will be able to intercept it)
        // select! on receivers for result of jobs 
    } 
}
