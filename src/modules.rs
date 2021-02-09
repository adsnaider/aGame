pub mod communication;

use std::collections::HashMap;

pub struct ModuleManager<M> {
    modules: HashMap<String, ModuleInfo<M>>,
}

struct ModuleInfo<M> {
    module: Box<dyn Module<Message = M>>,
    channel: communication::Channel<M>,
}

pub trait Module {
    type Message;
    fn entry(&self, sender: communication::Sender<Self::Message>);
    fn name(&self) -> String;
}

impl<M> ModuleManager<M> {
    pub fn new() -> ModuleManager<M> {
        ModuleManager {
            modules: HashMap::new(),
        }
    }
    pub fn register_module(&mut self, module: impl Module<Message = M> + 'static) {
        let channel = communication::Channel::new();
        let info = ModuleInfo {
            module: Box::new(module),
            channel,
        };
        self.modules.insert(info.module.name(), info);
    }

    pub fn observe(&self, module: &str) -> Option<communication::Receiver<M>> {
        if let Some(info) = self.modules.get(module) {
            return Some(info.channel.receiver().clone());
        }
        None
    }

    pub fn start() {}
}
