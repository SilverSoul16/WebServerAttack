use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::time::Duration;
use web_server::ThreadPool;
use std::thread;

pub fn init(args: Vec<String>) {
    let mut max = args.len() - 1;
    let mut counter = 1;
    let mut root: &str = "127.0.0.1";
    let mut port: &str = "7878";
    let mut process_n: usize = 1;

    while max > 0 {
        let flag = &args[counter];
        let flag_value = &args[counter + 1];
        
        if flag == "-n" {
            process_n = flag_value.parse().unwrap();
        } else if flag == "-w" {
            root = flag_value;
        } else if flag == "-p"{
            port = flag_value;
        } else {
            println!("ERROR: invalid flag. Valid options: -n -w -p");
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

        pool.execute(|| {
            handle_requests(stream);
        });
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