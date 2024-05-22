use moxie::{
    config::load_configuration,
    service::{controller::IngressController, http_proxy::HttpProxy},
    telemetry::init_logger,
};

#[tokio::main]
async fn main() {
    // Initialization
    init_logger();
    load_configuration();

    // Start the HttpProxy in a blocking thread
    let http_proxy = HttpProxy::new();
    let proxy_handle = tokio::task::spawn_blocking(move || {
        http_proxy.serve(); // Assuming serve is a blocking function
    });

    // Start the IngressController asynchronously
    let ingress_handle = tokio::spawn(async {
        let mut ingress_controller = IngressController::new();
        ingress_controller.start_controller().await;
    });

    // Await both tasks
    let _ = tokio::try_join!(ingress_handle, proxy_handle);
}
