pub mod child1;
pub mod child2;

pub struct Parent {
    // name 和 age两个字段必须声明pub，否则在模块外无法访问
    // 如果不加pub，那只能在模块内访问
    pub name: String,
    pub age: i32,
}
// public方法，模块外可以访问
pub fn public_function() {
    println!("public function");
}
// private方法，模块外不能访问
fn private_function() {
    println!("private function");
}