
// trait是 Rust 体系中的接口或抽象基类，它和 Java 中的接口差不多。
pub trait Runner {
    fn run(&self);

    fn callout(&self) {
        println!("A animal calling")
    }
}