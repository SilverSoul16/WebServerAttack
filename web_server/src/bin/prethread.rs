extern crate ftp;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::time::Duration;
use web_server::ThreadPool;
use std::thread;
use std::process::Command;

static mut DATABASE: Vec<serde_json::Value> = Vec::new();

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

        //sends each request to a different thread
        pool.execute(|| {
            handle_requests(stream);
        });
    }
}

pub fn client_init(args: Vec<String>) {
    let mut max = args.len() - 1;
    let mut counter = 1;
    let mut host_to_connect: &str = "localhost:7878/";

    while max > 0 {
        let flag = &args[counter];
        let flag_value = &args[counter + 1];
        
        if flag == "-h" {
            host_to_connect = flag_value;
        } else if flag.starts_with("["){
            let comm_list = &args[counter..];
            let method = &args[counter + 1].trim_end_matches("]").to_string();
            format_command(comm_list.to_vec(), host_to_connect, method);

            break;
        } else {
            println!("ERROR: invalid flag. Valid options: -h");
            break;
        }
        max -= 2;
        counter += 2;
    }

}

fn format_command(mut commands: Vec<String>, host: &str, method: &str) {
    // HTTPclient [-X PUT -d @test.txt]
    // HTTPclient [-X DELETE test.txt]
    // HTTPclient [-X GET]
    // HTTPclient [-X HEAD]
    let last = commands.len() - 1;
    commands[0] = commands[0].trim_start_matches("[").to_string();
    commands[last] = commands[last].trim_end_matches("]").to_string();

    if method == "PUT" {
        let url = format!("{}{}", host.to_string(), commands[last].trim_start_matches("@"));
        commands.insert(0, url);
    } else if method == "HEAD" {
        commands = vec!["-I".to_string(), host.to_string()];
    } else if method == "GET" || method == "POST"{
        commands.insert(0, host.to_string());
    } else if method == "DELETE" {
        let url = format!("{}{}", host.to_string(), commands[last]);
        commands.insert(last, url);
    }

    let output = Command::new("curl")
                        .args(&commands)
                        .output()
                        .unwrap()
                        .stdout;

    let out = std::str::from_utf8(&output).unwrap(); //Vec<u8> to &str

    for charac in out.chars(){
        print!("{}", charac)
    }
    
}

fn handle_requests(mut stream: TcpStream){
    let mut buffer = [0; 1024];         // holds the data as its read on, 1024 bytes
    stream.read(&mut buffer).unwrap();

    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let buffer_vector: Vec<String> = std::str::from_utf8(&buffer).unwrap().split(' ').map(|s| s.to_string()).collect(); // creates a vector from args

    if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        get("HTTP/1.1 200 OK", stream);
    } else if buffer.starts_with(b"GET") || buffer.starts_with(b"POST") {
        get("HTTP/1.1 200 OK", stream);
    } else if buffer.starts_with(b"PUT") {
        put("HTTP/1.1 200 OK", buffer_vector[1].trim_start_matches("/"), stream);
    } else if buffer.starts_with(b"DELETE") {
        delete("HTTP/1.1 200 OK", buffer_vector[1].trim_start_matches("/"), stream);
    } else if buffer.starts_with(b"HEAD") {
        head("HTTP/1.1 200 OK", stream);
    } else {
        println!("ERROR: Invalid command.");
    }
}


fn get(status_line: &str, mut stream: TcpStream){
    unsafe {
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{:#?}",
            status_line,
            DATABASE.len(),
            DATABASE
        );

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("Response: {}", response),
            Err(e) => println!("ERROR: {}", e),
        }
        stream.flush().unwrap();
    }
}

fn put(status_line: &str, file: &str, mut stream: TcpStream) {
    let data = serde_json::json!({
        "title": file,
        "contents": fs::read_to_string(file).unwrap()
    });

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        serde_json::to_string(&data).unwrap().len(),
        serde_json::to_string_pretty(&data).unwrap()
    );

    unsafe {
        DATABASE.push(data);
    }

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response: {}", response),
        Err(e) => println!("ERROR: {}", e),
    }
    stream.flush().unwrap();
}


fn delete(status_line: &str, file: &str, mut stream: TcpStream) {
    unsafe {
        for i in 0..DATABASE.len() {
            println!("{}", DATABASE[i]["title"]);
            if DATABASE[i]["title"] == file {
                DATABASE.remove(i);
            }
        }
    

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{:#?}",
            status_line,
            DATABASE.len(),
            DATABASE
        );

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("Response: {}", response),
            Err(e) => println!("ERROR: {}", e),
        }
        stream.flush().unwrap();
    }
}

fn head(status_line: &str, mut stream: TcpStream){
    unsafe {
        let response = format!("{}\r\n", status_line);

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("Response: {}", response),
            Err(e) => println!("ERROR: {}", e),
        }
        stream.flush().unwrap();
    }
}