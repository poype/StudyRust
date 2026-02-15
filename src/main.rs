mod study_trait;

fn main() {
    let s1 = "long string ~~~~~".to_string();
    {
        let s2 = "short str".to_string();
        let result = longest(&s1, &s2);
        println!("result: {}", result);
    }
}

// s1和s2的生命周期参数相同，所以这里的'a参数表示的生命周期只能是s1和s2的交集，返回值的生命周期也只能在这个交集中
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}