use std::{sync::{Arc, Mutex, mpsc}, thread};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult, Pid};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker{
    id: usize,
    child_pid: Pid
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("[main] Hi there! My PID is {}.", getpid());

        let child_pid = match fork() {
            Ok(ForkResult::Child) => {
                println!("[child] Antes del lock.");
                let job = receiver.lock().unwrap().recv().unwrap(); //receive a job from the channel

                println!("[child] Despues del receive.");
                //println!("Worker {} got a job; executing.", id);
                println!(
                    "[child] I'm alive! My PID is {} and PPID is {}.",
                    getpid(),
                    getppid()
                );

                println!("[child] Antes del JOB.");
                job();
                println!("[child] Despues del JOB.");

                exit(0);
            }

            Ok(ForkResult::Parent { child, .. }) => {
                println!("[main] I forked a child with PID {}.", child);
                child
            }
    
            Err(err) => {
                panic!("[main] fork() failed: {}", err);
            }
        };

        // Nose si funciona con este código aqui
        println!("[main] I'll be waiting for the child termination...");
        match waitpid(child_pid, None) {
            Ok(status) => println!("[main] Child exited with status {:?}", status),
            Err(err) => panic!("[main] waitpid() failed: {}", err),
        }
        println!("[main] Bye Bye!");

        Worker { id, child_pid }
    }
}