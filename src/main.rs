mod study_trait;

use trpl::{Html};

// 普通main函数，在main函数内部嵌入了一个async block
fn main() {
    trpl::run(
        async {
            let url = "https://www.baidu.com/";
            let title = page_title(url).await.unwrap();

            println!("title: {title}"); // title: 百度一下，你就知道
        }
    )
}

// async说明这个函数是一个异步函数
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}