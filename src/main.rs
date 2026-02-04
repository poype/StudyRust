mod parent;

// 引入模块中的属性
use parent::child2::child2_function;
use parent::child1;

fn main() {
    // 直接调用child2模块中的public方法
    child2_function();
    
    // 可以通过名字直接调
    parent::child2::child2_function();

    // 通过child1模块的名字调用其中的public方法
    child1::child1_function();
}
