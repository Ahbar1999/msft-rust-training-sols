use std::sync::{Arc, Mutex, mpsc};
use std::thread;
// use crossbeam_channel::{select};

/*
 * Implementation NOTES(after my implementaion to compare against):
 *  we are creating a separate channel for each worker thread instead should use a single result
 *      channel shared the same way as job channel among all worker threads 
 *  
 *  each thread should block until it gets a job and terminate after doing that job
 *
 *  a worker pool function takes in jobs and creates a pool of workers and give jobs to each worker
 *  and returns the result which is stored in the Worker struct.  
 * */

pub struct Job {
    pub id: usize,
    pub done: bool 
}

impl Job {
    pub fn new(id: usize) -> Self {
        Job {
            id,
            done: false
        }
    }
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
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> (thread::JoinHandle<()>, mpsc::Receiver<Result<String, String>>) {
        let (result_tx, result_rx) = mpsc::channel();
        // create a new worker 
        let mut worker = Worker {
            is_busy: false,
            id,
            job_rx: rx,
            result_tx
        };

        let handle =thread::spawn(move || {
            println!("worker #{:?} created.", id);
            worker.run();
        });

        (handle, result_rx)
    } 

    pub fn run(&mut self) {
        loop {
            let mut has_job = None;
            // check if there is any job to recv
            if let Ok(rx_locked) = self.job_rx.lock() {
                if let Ok(new_job) = rx_locked.try_recv() {
                    has_job = Some(new_job);
                }
            } // release lock regardless of wethere we got a job or not
        
            if let Some(mut job) = has_job {
                self.is_busy = true;
                println!("worker #{:?} received job #{:?}", self.id, job.id);
                
                thread::sleep_ms(100);
                
                // finish job 
                job.done = true;

                // println!("worker #{:?} finished job #{:?}", self.id, job.id);
                self.result_tx.send(Ok(format!("worker #{:?} finished job #{:?}", self.id, job.id))).ok();
                
                self.is_busy = false;
            }
        } 
    }
}

#[allow(unreachable_code)]
fn main() {
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let job_rx_ref = Arc::new(Mutex::new(job_rx));
    
    let mut workers = Vec::<(thread::JoinHandle<()>, mpsc::Receiver<Result<String, String>>)>::new();
    // fill worker pool of 10 workers
    for id in 0..10 {
        workers.push(Worker::new(id, job_rx_ref.clone()));
    }

    // create some jobs
    // send each job through job_tx(some worker will be able to intercept it)
    // select! on receivers for result of jobs 
    for job_id in 0..100 {
        job_tx.send(Job::new(job_id)).ok(); 
    }
    
    loop {
        for (_, rx) in workers.iter() {
            if let Ok(msg) = rx.try_recv() {
               println!("{:?}", msg);  
            }
        } 
    }
    
    /*
    // wait for thread to finish() 
    for (join_handle, _) in workers {
        join_handle.join().ok();
    }
    */
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn basics() {
    } 
}

*/
