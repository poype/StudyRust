// as_ref()、as_deref()
// as_mut()、as_deref_mut()，as_mut与as_ref对应，as_deref_mut与as_deref对应
// take

fn main() {
    // String <==> &str
    let option_string: Option<String> = Some(String::from("test"));

    let option_ref_string: Option<&String> = option_string.as_ref();

    let option_ref_str: Option<&str> = option_string.as_deref();

    // Box <==> direct ref
    let option_box: Option<Box<i32>> = Some(Box::new(4));

    let option_ref_box: Option<&Box<i32>> = option_box.as_ref();

    let option_ref_box: Option<&i32> = option_box.as_deref();

    // Vec <==> &[T]
    let option_vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);

    let option_ref_vec: Option<&Vec<i32>> = option_vec.as_ref();

    let option_ref_slice: Option<&[i32]> = option_vec.as_deref();

    // take方法 把 Option 里的值 “取走、拿走”，原来的 Option 变成 None，拿走的值作为新 Option 返回。
    let mut option_num: Option<i32> = Some(4);
    let option_num2: Option<i32>  = option_num.take();

    assert_eq!(option_num, None);
    assert_eq!(option_num2, Some(4));
}