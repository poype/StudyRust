mod study_trait;
use crate::study_trait::animal::{Cat, Dog};
use crate::study_trait::runner::Runner;

fn main() {
    let cat = Cat::new("Jack".to_string(), "red".to_string());
    let dog = Dog::new("Chris".to_string(), "Kingdom".to_string());

    cat.run();  // A cat whose name is Jack color is red is running!!!
    dog.run();  // A dog of Kingdom whose name is Chris is quickly running!!!

    // 调用trait中的默认方法实现
    cat.callout();
    dog.callout();
}