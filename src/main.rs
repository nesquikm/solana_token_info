use log::info;

mod cli;
mod logger;
mod request;
mod endpoint;

#[tokio::main]
async fn main() {
    let cli = cli::parse_cli();

    logger::init(cli.debug_level);

    info!("The address is: {}", cli.address);
    info!("Endpoint: {}", cli.endpoint);

    let token_info = request::get_info(cli.endpoint, cli.address).await;

    match token_info {
        Ok(token_info) => {
            println!("{}", token_info);
        }
        Err(e) => {
            print!("Something went wrong: {:?}", e);
        }
    }
}
