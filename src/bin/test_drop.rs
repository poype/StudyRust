fn main() {
    let p = Person {
        name: "Chris".to_string(),
        age: 42,
    };

    test(p);

    println!("end of main");
}

// 按顺序打印如下三行日志
// received a person whose name is Chris
// Chris is being dropped
// end of main

fn test(p: Person) {
    println!("received a person whose name is {}", p.name);
}

struct Person {
    name: String,
    age: i32,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("{} is being dropped", self.name);
    }
}