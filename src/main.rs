use std::io;

fn main() {
    //標準入力から受け取り
    eprint!("サーバーのアドレスを入力してください : ");
    let mut server_address = String::new();

    io::stdin().read_line(&mut server_address).ok();

    eprint!("サーバーのサブネットマスクを入力してください : ");
    let mut server_subnet = String::new();

    io::stdin().read_line(&mut server_subnet).ok();

    eprint!("クライアントのアドレスを入力してください : ");
    let mut client_address = String::new();

    io::stdin().read_line(&mut client_address).ok();

    eprint!("クライアントのサブネットマスクを入力してください : ");
    let mut client_subnet = String::new();

    io::stdin().read_line(&mut client_subnet).ok();

    //入力されたアドレス、サブネットを標準出力
    eprint!("\nサーバーアドレス: {}", server_address);
    eprint!("サーバーサブネット: {}", server_subnet);

    eprint!("\nクライアントアドレス: {}", client_address);
    eprint!("クライアントサブネット: {}", client_subnet);

    //ベクタ型（文字列）で保存
    let server_address: Vec<&str> = server_address.trim().split('.').collect();
    let server_subnet: Vec<&str> = server_subnet.trim().split('.').collect();
    let client_address: Vec<&str> = client_address.trim().split('.').collect();
    let client_subnet: Vec<&str> = client_subnet.trim().split('.').collect();

    //上のベクタ型をクローン
    let server_address_clone = server_address.clone();
    let server_subnet_clone = server_subnet.clone();
    let client_address_clone = client_address.clone();
    let client_subnet_clone = client_subnet.clone();

    //ベクタ型のドットを外し、数値に変換
    let result_server = calculate_net(server_address, server_subnet);
    let connection_server_clone = calculate_net(client_address_clone, server_subnet_clone);
    let result_client = calculate_net(client_address, client_subnet);
    let connection_client_clone = calculate_net(server_address_clone, client_subnet_clone);

    //サーバ視点のネットワーク部を10進数表記に変換
    let result_server = shift_address(result_server);
    let result_server_clone = shift_address(connection_server_clone);

    //クライアント視点のネットワーク部を10進数表記に変換
    let result_client = shift_address(result_client);
    let result_client_clone = shift_address(connection_client_clone);

    //結果出力
    println!("\nサーバー視点 : ");
    eprint!("\t\tサーバNetAddr\t    : {}", result_server.join("."));
    println!(
        "\n\t\tクライアントNetAddr : {}",
        result_server_clone.join(".")
    );
    println!("\nクライアント視点 : ");
    eprint!("\t\tサーバNetAddr\t    : {}", result_client.join("."));
    println!(
        "\n\t\tクライアントNetAddr : {}\n",
        result_client_clone.join(".")
    );

    //接続可能か比較(文字列型のまま)
    if result_server == result_client && result_server_clone == result_client_clone {
        println!("接続可能！");
    } else {
        println!("接続不可！");
    }
}

//シフトして＆演算でネットワーク部を求める
fn calculate_net(addr: Vec<&str>, sub: Vec<&str>) -> i64 {
    let mut address: i64 = 0;
    let mut subnet: i64 = 0;
    let mut j: i32 = 3;
    let mut y: i32 = 3;
    let result: i64;

    for i in &addr {
        while j != -1 {
            let shift_num = j * 8;

            address += i.parse::<i64>().unwrap() << shift_num;
            j -= 1;
            break;
        }
    }
    for m in &sub {
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

//ネットワーク部を10進数表記に変換
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
