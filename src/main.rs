use std::sync::mpsc;

#[derive(PartialEq)]
enum MyInfo {
    FOO,
}

struct MyModule {}

impl agame::modules::Module for MyModule {
    type Message = MyInfo;
    fn entry(&self, sender: agame::modules::communication::Sender<Self::Message>) {
        println!("Hello");
    }

    fn name(&self) -> String {
        "My Module".to_string()
    }
}

fn main() {
    let mut manager = agame::modules::ModuleManager::new();
    let module = MyModule {};

    manager.register_module(module);
}
