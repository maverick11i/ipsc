use std::io;
#[derive(Debug)]
struct Network {
    address: String,
    subnet: String,
}

impl Network {
    fn input_server(&mut self) {
        eprint!("サーバーのアドレスを入力してください : ");
        io::stdin()
            .read_line(&mut self.address)
            .expect("Please type a ip address!!!");

        eprint!("サーバーのサブネットマスクを入力してください : ");
        io::stdin()
            .read_line(&mut self.subnet)
            .expect("Please type a subnetmask!!!");
    }

    fn input_client(&mut self) {
        eprint!("クライアントのアドレスを入力してください : ");
        io::stdin()
            .read_line(&mut self.address)
            .expect("Please type a ip address!!!");

        eprint!("クライアントのサブネットマスクを入力してください : ");
        io::stdin()
            .read_line(&mut self.subnet)
            .expect("Please type a subnetmask!!!");
    }
}

fn result(server: Network, client: Network) {
    fn calc_net(addr: &Vec<&str>, sub: &Vec<&str>) -> i64 {
        let mut address: i64 = 0;
        let mut subnet: i64 = 0;
        let mut j: i32 = 3;
        let mut y: i32 = 3;
        let result: i64;

        for i in addr {
            while j != -1 {
                let shift_num = j * 8;

                address += i.parse::<i64>().unwrap() << shift_num;
                j -= 1;
                break;
            }
        }
        for m in sub {
            while y != -1 {
                let shift_num = y * 8;

                subnet += m.parse::<i64>().unwrap() << shift_num;
                y -= 1;
                break;
            }
        }
        result = address & subnet;
        result
    }

    fn shift_address(net_add: i64) -> Vec<String> {
        let mut i: i32 = 3;
        let mut result: i64;
        let mut result_str: Vec<String> = Vec::new();

        while i != -1 {
            let shift_num = i * 8;

            result = (net_add >> shift_num) & 255;
            result_str.push(result.to_string());
            i -= 1;
        }
        result_str
    }

    let server_address: Vec<&str> = server.address.trim().split('.').collect();
    let server_subnet: Vec<&str> = server.subnet.trim().split('.').collect();
    let client_address: Vec<&str> = client.address.trim().split('.').collect();
    let client_subnet: Vec<&str> = client.subnet.trim().split('.').collect();

    let result_server = calc_net(&server_address, &server_subnet);
    let connection_server = calc_net(&client_address, &server_subnet);
    let result_client = calc_net(&client_address, &client_subnet);
    let connection_client = calc_net(&server_address, &client_subnet);

    let result_server = shift_address(result_server);
    let connection_server = shift_address(connection_server);
    let result_client = shift_address(result_client);
    let connection_client = shift_address(connection_client);

    println!("\nサーバー視点 : ");
    eprint!("\t\tサーバNetAddr\t    : {}", result_server.join("."));
    println!(
        "\n\t\tクライアントNetAddr : {}",
        connection_server.join(".")
    );
    println!("\nクライアント視点 : ");
    eprint!("\t\tサーバNetAddr\t    : {}", result_client.join("."));
    println!(
        "\n\t\tクライアントNetAddr : {}\n",
        connection_client.join(".")
    );

    if result_server == result_client && connection_server == connection_client {
        println!("接続可能！");
    } else {
        println!("接続不可！");
    }
}

fn main() {
    let mut server = Network {
        address: String::new(),
        subnet: String::new(),
    };

    let mut client = Network {
        address: String::new(),
        subnet: String::new(),
    };

    server.input_server();
    client.input_client();

    result(server, client);
}
