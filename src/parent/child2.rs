use crate::parent::private_function;

pub fn child2_function() {
    println!("child2 function");
    // 子module可以调用父module中的私有方法
    private_function();
}