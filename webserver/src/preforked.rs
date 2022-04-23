use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

use std::process::{exit};
use std::thread::{sleep};

use nix::libc::{prctl, PR_SET_CHILD_SUBREAPER};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult, getpid, getppid, Pid};

use std::time::Duration;
use webserver::ThreadPool;
use std::thread;

pub fn init(args: Vec<String>) {
    //let args: Vec<String> = env::args().collect();  // get args of command line

    let mut max = args.len() - 1;
    let mut counter = 1;
    let mut root: &str = "127.0.0.1";
    let mut port: &str = "7878";
    let mut process_n: usize = 1;

    while max > 0 {
        let flag = &args[counter];
        let flag_value = &args[counter + 1];
        
        if flag == "-n" {
            // código para hacer el pool de procesos
            println!("cantidad de procesos: {}", flag_value);
            process_n = flag_value.parse().unwrap();
        } else if flag == "-w" {
            root = flag_value;
        } else if flag == "-p"{
            port = flag_value;
        } else {
            println!("invalid flag");
            break;
        }
        max -= 2;
        counter += 2;
    }

    // initializes the server
    server_init(root, port, process_n);
}

fn server_init(root: &str, port: &str, n: usize) {
    let http_url = format!("{}:{}", root, port);
    println!("{}", http_url);

    let listener = TcpListener::bind(http_url).unwrap();        // returns a new tcp listener, returns Result<T, E>
    let pool = ThreadPool::new(n);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // aqui se manda a llamar al pool y se ejecuta el handle_request
        pool.execute(|| {
            handle_requests(stream);  //takes a closure and gives it to a thread in the pool to execute
        })
    }
}

fn handle_requests(mut stream: TcpStream){
    let mut buffer = [0; 1024];         // holds the data as its read on, 1024 bytes
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hi.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hi.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn process_pool() {
    println!("[main] Hi there! My PID is {}.", getpid());
    println!("[main] Making myself a child subreaper.");
    unsafe {
        prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }

    match fork() {
        Ok(ForkResult::Child) => {
            println!("[child 1] I'm alive! My PID is {} and PPID is {}.", getpid(), getppid());

            match fork() {
                Ok(ForkResult::Child) => {
                    //////////////////////
                    //      child 2     //
                    //////////////////////
                    for _ in 0..6 {
                        println!("[child 2] I'm alive! My PID is {} and PPID is {}.", getpid(), getppid());
                        sleep(Duration::from_millis(500));
                    }
                    println!("[child 2] Bye Bye");
                    exit(0);
                }

                Ok(ForkResult::Parent { child, .. }) => {
                    println!("[child 1] I forked a grandchild with PID {}.", child);
                }

                Err(err) => panic!("[child 1] fork() failed: {}", err),
            };

            println!("[child 1] I'm gonna sleep for a while and then just exit...");
            sleep(Duration::from_millis(1500));
            exit(0);
        }

        Ok(ForkResult::Parent { child, .. }) => {
            println!("[main] I forked a child with PID {}.", child);
        }

        Err(err) => panic!("[main] fork() failed: {}", err),
    };

    println!("[main] I'll be waiting for the child termination...");
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[main] Child exited with status {:?}", status),
        Err(err) => println!("[main] waitpid() failed: {}", err),
    }

    println!("[main] I'll be waiting for the grandchild termination as well...");
    sleep(Duration::from_millis(500));  // just in case
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("[main] Grandchild exited with status {:?}", status),
        Err(err) => println!("[main] waitpid() failed: {}", err),
    }
    println!("[main] Bye Bye!");
}

// rustc ./src/preforked-WebServer.rs 
// ./preforked-WebServer -n [cantidad de procesos] -w [http-root] -p [port]

