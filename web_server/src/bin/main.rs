mod prethread;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter command (write exit to end): ");
        let mut _input_bytes = std::io::stdin().read_line(&mut input).unwrap();
        if input.starts_with("prethread-WebServer"){
            input = input.trim().to_string(); //removes \n from read_line
            let input_vector: Vec<String> = input.split(' ').map(|s| s.to_string()).collect(); // creates a vector from args
            prethread::init(input_vector);
            // prethread-WebServer -n cantidad -w http-root -p port
        } else if input.starts_with("preforked-WebServer"){
            // code for preforked
            // preforked-WebServer -n cantidad -w http-root -p port
        } else if input.starts_with("HTTPclient"){
            input = input.trim().to_string(); //removes \n from read_line
            let input_vector: Vec<String> = input.split(' ').map(|s| s.to_string()).collect(); // creates a vector from args
            prethread::client_init(input_vector);
            // HTTPCLient -h <host a conextar> [lista de comandos a ejecutar]
        } else if input.starts_with("stress"){
            input = input.trim().to_string(); //removes \n from read_line

            while true {
                let input_vector: Vec<String> = input.split(' ').map(|s| s.to_string()).collect(); // creates a vector from args
                prethread::client_init(input_vector);
            }
            
            //stress [lista de comandos a ejecutar]
        } else if input.starts_with("exit"){
            //break;
        } else {
            println!("Invalid command");
        }
    }
}
