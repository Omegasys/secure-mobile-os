use super::dependency_graph::ServiceGraph;

pub struct Service {
    pub name: &'static str,
    pub running: bool,
}

pub struct ServiceManager {
    services: [Option<Service>; 128],
    graph: ServiceGraph,
}

impl ServiceManager {
    pub const fn new() -> Self {
        Self {
            services: [None; 128],
            graph: ServiceGraph::new(),
        }
    }

    pub fn register_service(&mut self, name: &'static str) {
        for slot in self.services.iter_mut() {
            if slot.is_none() {
                *slot = Some(Service {
                    name,
                    running: false,
                });
                break;
            }
        }
    }

    pub fn start_service(&mut self, name: &'static str) {
        for service in self.services.iter_mut().flatten() {
            if service.name == name {
                service.running = true;
            }
        }
    }

    pub fn stop_service(&mut self, name: &'static str) {
        for service in self.services.iter_mut().flatten() {
            if service.name == name {
                service.running = false;
            }
        }
    }
}
