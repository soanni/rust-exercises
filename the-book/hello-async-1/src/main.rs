use trpl::Either;
use trpl::Html;

async fn page_title(url: &str) -> (&str, Option<String>) {
    //let response = trpl::get(url).await;
    //let response_text = response.text().await;
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    trpl::block_on(async {
        let future_title_left = page_title(&args[1]);
        let future_title_right = page_title(&args[2]);

        let (url, maybe_future) = match trpl::select(future_title_left, future_title_right).await {
            Either::Left(l) => l,
            Either::Right(r) => r,
        };

        println!("page '{url}' scrape finished first");

        match maybe_future {
            Some(t) => println!("the title for page '{}' is {}", url, t),
            None => println!("no title for the page '{}'", url),
        }
    });
}
