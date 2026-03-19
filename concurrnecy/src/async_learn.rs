use trpl::Html;

pub async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

pub fn page_two() {
    trpl::block_on(async {
        use trpl::Either;
        let title_fut_1 = crate::async_learn::page_title("https://www.rust-lang.org");
        let title_fut_2 = crate::async_learn::page_title("https://www.google.com");

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}

use std::time::Duration;

pub fn spawn() {
    // 提供一个运行时
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        handle.await.unwrap();

        let handle2 = trpl::spawn_task(async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        handle2.await.unwrap();

        // handle.await.unwrap();
    });
}
