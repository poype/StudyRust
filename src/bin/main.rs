
use trpl::{Html};

fn main() {
    let url1 = "https://www.baidu.com";
    let url2 = "https://www.bilibili.com";

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .thread_stack_size(5 * 1024 * 1024)
        .event_interval(20)
        .max_blocking_threads(256)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(page_title(url1));
    runtime.block_on(page_title(url2));
}

async fn page_title(url: &str) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    println!("title: {}", title.unwrap());
}
