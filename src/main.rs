use skyscanner::configuration::get_configuration;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Can't get the configuration.");
}
