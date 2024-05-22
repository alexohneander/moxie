use tracing::info;

const INGRESS_CLASS_NAME: &'static str = "moxie";

pub struct IngressController{
}

impl IngressController {
    pub fn new() -> IngressController{
        return IngressController{};
    }

    pub fn start_controller(&mut self){
        info!("starting controller");
        self.watch_ingress_resources();
    }
    
    fn watch_ingress_resources(&self){
        info!(INGRESS_CLASS_NAME, "watching ingress resources with: ");
    }
}

