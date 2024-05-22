use tracing::info;
use tracing_subscriber;

pub fn init_logger(){
    tracing_subscriber::fmt::init();
    info!("initilize logging")
}