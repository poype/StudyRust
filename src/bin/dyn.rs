use std::sync::{mpsc, Arc, Mutex};

// dyn 运行期多肽，它的原理是vtable
fn main() {
    let listener1 = ChangeListenerImpl1{};
    let listener2 = ChangeListenerImpl2{};

    // dyn 和 Rc&Arc 联合使用
    test_dyn1(Arc::new(listener1));
    test_dyn1(Arc::new(listener2));

    println!("----------------------------------");

    // dyn和引用联合使用
    let listener1 = ChangeListenerImpl1{};
    let listener2 = ChangeListenerImpl2{};
    test_dyn2(&listener1);
    test_dyn2(&listener2);

    println!("----------------------------------");

    // dyn 和 Box联合使用
    let listener1 = ChangeListenerImpl1{};
    let listener2 = ChangeListenerImpl2{};
    test_dyn3(Box::new(listener1));
    test_dyn3(Box::new(listener2));
}

trait ChangeListener {

    fn on_change(&self, response: String);
}

struct ChangeListenerImpl1 {}

impl ChangeListener for ChangeListenerImpl1 {
    fn on_change(&self, response: String) {
        println!("1111 {}", response);
    }
}

struct ChangeListenerImpl2 {}

impl ChangeListener for ChangeListenerImpl2 {
    fn on_change(&self, response: String) {
        println!("2222 {}", response);
    }
}

fn test_dyn1(listener: Arc<dyn ChangeListener>) {
    listener.on_change("test message1".to_string());
}

fn test_dyn2(listener: &dyn ChangeListener) {
    listener.on_change("test message2".to_string());
}

fn test_dyn3(listener: Box<dyn ChangeListener>) {
    listener.on_change("test message3".to_string());
}