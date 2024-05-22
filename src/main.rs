use moxie::{config::load_configuration, service::{controller::IngressController, http_proxy::HttpProxy}, telemetry::init_logger};

fn main() {
    // Initilization 
    init_logger();
    load_configuration();
    
    // Start controller
    let mut ingress_controller = IngressController::new();
    ingress_controller.start_controller();

    // Start Proxy/Gateway
    let mut http_proxy = HttpProxy::new();
    http_proxy.serve();
}