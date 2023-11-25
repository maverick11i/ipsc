use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    arp: Option<String>,

    #[arg(short, long)]
    ping: Option<String>,

    #[arg(short, long)]
    can_communication: Option<String>,

    #[arg(short, long)]
    show_macaddress: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        // test options
        if let Some(arp) = cli.arp.as_deref() {
            println!("{}", arp);
        }

        if let Some(ping) = cli.ping.as_deref() {
            println!("{}", ping);
        }

        if let Some(can_communication) = cli.can_communication.as_deref() {
            println!("{}", can_communication);
        }

        if let Some(show_macaddress) = cli.show_macaddress.as_deref() {
            println!("{}", show_macaddress);
        }

        match &cli.command {
            Some(Commands::Test { list }) => {
                if *list {
                    println!("true list...");
                } else {
                    println!("false list...");
                }
            }
            None => {}
        }
    }
}
