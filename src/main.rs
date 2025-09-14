use trpl::{Either, Html};

// takes a url string, makes a request to it and
// returns the text of the title element
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url1 = page_title(&args[1]);
        let url2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(url1, url2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("It's page title is '{title}'"),
            None => println!("It's title could not be parsed."),
        }
    })
}
