mod prethread;
mod http_client;
use std::process::Command;

fn main() {
    //while true {
        let mut input = String::new();
        println!("Enter command (write exit to end): ");
        let mut input_bytes = std::io::stdin().read_line(&mut input).unwrap();
        if input.starts_with("prethread-WebServer"){
            let mut input = input.trim(); //removes \n from read_line
            let input_vector: Vec<String> = input.split(' ').map(|s| s.to_string()).collect(); // creates a vector from args
            println!("vector: {:?}", input_vector);
            prethread::init(input_vector);
            // preforked-WebServer -n cantidad -w http-root -p port
        } else if input.starts_with("preforked-WebServer"){
            // code for preforked
            // preforked-WebServer -n cantidad -w http-root -p port
        } else if input.starts_with("HTTPclient"){
            let mut input = input.trim(); //removes \n from read_line
            let input_vector: Vec<String> = input.split(' ').map(|s| s.to_string()).collect(); // creates a vector from args
            println!("vector: {:?}", input_vector);
            http_client::init(input_vector);
            // HTTPCLient -h <host a conextar> [lista de comandos a ejecutar]
        } else if input.starts_with("stress"){
            Command::new("stress.py")
            .output();
            //.expect("failed to execute process");
        } else if input.starts_with("exit"){
            //break;
        } else {
            println!("Invalid command");
        }
    //}
}
