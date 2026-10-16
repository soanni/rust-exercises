use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    //let response = trpl::get(url).await;
    //let response_text = response.text().await;
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(t) => println!("the title for page {} is {}", url, t),
            None => println!("no title for the page {}", url),
        }
    });
}
