use std::vec::Vec;

/* 
 * EPIC FAIL!!!!
 * MY IMPLMENTATION helped me understand that if you want to have shared + mutable + in sync access
 *  to data then its very hard to achieve those 3 thing simulataneously, basically the threads will
 *  need to run in step lock fashion which just defeats the whole purpose of having parallel
 *  execution;
 * */
/*
fn parallel_map<T, R>(data: &[T], f: fn(&T) -> R, num_threads: usize) -> Vec<R> 
    where T: Debug + Sync + Send + Clone, R: Sync + Send {
    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    let ret_vec_lk = Arc::new(Mutex::new(Vec::with_capacity(data.len())));
    let (cv, cv_mtx) = (Arc::new(Mutex::new(1)), Condvar::new());

    
    std::thread::scope(|s| {
        for (i, chunk) in data.chunks(chunk_size).enumerate() {
            let this_ret_vec_lk = ret_vec_lk.clone();

            s.spawn(move || {
                loop {
                    if let Ok(mut ret_vec) = this_ret_vec_lk.try_lock() {
                        for ele in chunk {
                            print!("{:?} ", ele);
                            ret_vec.push(f(ele));
                        }
                        break;
                    } else {
                        continue;
                    }
                }
            });
        }
    });

    Arc::try_unwrap(ret_vec_lk).ok().unwrap().into_inner().ok().unwrap() 
}
*/

fn main() {
    // println!("Hello, world!");
}
fn parallel_map<T: Sync, R: Send>(data: &[T], f: fn(&T) -> R, num_threads: usize) -> Vec<R> {
    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    let mut results = Vec::with_capacity(data.len());

    std::thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in data.chunks(chunk_size) {
            handles.push(s.spawn(move || {
                chunk.iter().map(f).collect::<Vec<_>>()
            }));
        }
        for h in handles {
            results.extend(h.join().unwrap());
        }
    });

    results
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::time::Instant;
        
    #[test]
    fn basics() {
        let mut data = Vec::new();
        for i in 0..10 {
            data.push(i);
        }

        let num_threads = 5;

        let mut start = Instant::now();
        let returned = parallel_map(&data, |element| { 2 * element } , num_threads);
        println!("parallel call: {:?}", start.elapsed());
        
        start = Instant::now();
        let result: Vec<i32> = data.iter().map(|ele| 2 * ele).collect(); 
        println!("single(lib) call: {:?}", start.elapsed());

        assert_eq!(result, returned);
    }
}


