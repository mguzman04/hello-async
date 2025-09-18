use std::time::Duration;

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
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi number {i} form the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 1..5 {
            println!("Hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}
