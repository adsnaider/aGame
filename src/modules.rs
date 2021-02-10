use std::thread;

pub mod communication;

pub trait Entry: Send {
    fn entry(&self);
}

pub struct ModuleManager {
    modules: Vec<Box<dyn Entry>>,
    threads: Vec<std::thread::JoinHandle<()>>,
}

impl ModuleManager {
    pub fn new() -> ModuleManager {
        ModuleManager {
            modules: Vec::new(),
            threads: Vec::new(),
        }
    }

    pub fn register_module(&mut self, module: Box<dyn Entry>) {
        self.modules.push(module);
    }

    pub fn start(&mut self) {
        while !self.modules.is_empty() {
            if let Some(module) = self.modules.pop() {
                self.threads.push(thread::spawn(move || {
                    module.entry();
                }));
            }
        }
    }

    pub fn join(self) {
        for t in self.threads {
            t.join().unwrap();
        }
    }
}
