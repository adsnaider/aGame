struct Printer {
    rx: std::sync::mpsc::Receiver<String>,
}
struct Reader {
    tx: std::sync::mpsc::Sender<String>,
}

impl latte::modules::Entry for Printer {
    fn entry(&self) {
        let name = self.rx.recv().unwrap();
        println!("Hello {}", name);
    }
}

impl latte::modules::Entry for Reader {
    fn entry(&self) {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        self.tx.send(buffer).unwrap();
    }
}

fn main() {
    let mut manager = latte::modules::ModuleManager::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let reader = Box::new(Reader { tx });
    let printer = Box::new(Printer { rx });
    manager.register_module(reader);
    manager.register_module(printer);
    manager.start();
    manager.join();
}
