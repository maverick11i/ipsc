use std::io;

fn main() {
    eprint!("サーバーのアドレスを入力してください :");
    let mut server_address = String::new();

    io::stdin()
        .read_line(&mut server_address)
        .expect("値が不正です");

    eprint!("サーバーのサブネットマスクを入力してください :");
    let mut server_subnet = String::new();

    io::stdin()
        .read_line(&mut server_subnet)
        .expect("値が不正です");

    eprint!("クライアントのアドレスを入力してください :");
    let mut client_address = String::new();

    io::stdin()
        .read_line(&mut client_address)
        .expect("値が不正です");

    eprint!("クライアントのサブネットマスクを入力してください :");
    let mut client_subnet = String::new();

    io::stdin()
        .read_line(&mut client_subnet)
        .expect("値が不正です");

    let server_address: Vec<&str> = server_address.trim().split('.').collect();
    let server_subnet: Vec<&str> = server_subnet.trim().split('.').collect();
    let client_address: Vec<&str> = client_address.trim().split('.').collect();
    let client_subnet: Vec<&str> = client_subnet.trim().split('.').collect();

    for i in server_address.iter() {
        let address = i.parse::<i32>().unwrap();
        eprint!("{:0>8b}", address);
    }
    for j in server_subnet.iter() {
        let address = j.parse::<i32>().unwrap();
        eprint!("{:0>8b}", address);
    }
    for x in client_address.iter() {
        let address = x.parse::<i32>().unwrap();
        eprint!("{:0>8b}", address);
    }
    for y in client_subnet.iter() {
        let address = y.parse::<i32>().unwrap();
        eprint!("{:0>8b}", address);
    }
}

// fn calc_address(server_address: Vec<&str>) -> i32 {
//     let address = server_address;
//     for i in address.iter() {
//         let readdress = i.parse::<i32>().unwrap()
//     }
// }
