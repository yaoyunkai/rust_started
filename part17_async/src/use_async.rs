// use trpl::{Either, Html};

// async fn page_title(url: &str) -> Option<String> {
//     let response = trpl::get(url).await;
//     let response_text = response.text().await;
//     Html::parse(&response_text)
//         .select_first("title")
//         .map(|title| title.inner_html())
// }

// async fn page_title_with_url(url: &str) -> (&str, Option<String>) {
//     let response_text = trpl::get(url).await.text().await;
//     let title = Html::parse(&response_text)
//         .select_first("title")
//         .map(|title| title.inner_html());
//     (url, title)
// }

// fn call_with_multi_urls() {
//     let url1 = "https://www.rust-lang.org";
//     let url2 = "https://www.baidu.com";

//     trpl::block_on(async {
//         let title_fut_1 = page_title_with_url(url1);
//         let title_fut_2 = page_title_with_url(url2);

//         let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
//             Either::Left(left) => left,
//             Either::Right(right) => right,
//         };

//         println!("{url} returned first");
//         match maybe_title {
//             Some(title) => println!("Its page title was: '{title}'"),
//             None => println!("It had no title."),
//         }
//     })
// }

// fn call_async_function() {
//     let url = "https://www.rust-lang.org";
//     // match page_title(url).await {
//     //     Some(title) => println!("The title for {url} was {title}"),
//     //     None => println!("{url} had no title"),
//     // }

//     trpl::block_on(async {
//         // let url = &args[1];
//         match page_title(url).await {
//             Some(title) => println!("The title for {url} was {title}"),
//             None => println!("{url} had no title"),
//         }
//     })
// }

// pub fn foo() {
//     hello_world::print("使用async", call_async_function);
//     hello_world::print("多个URL参数", call_with_multi_urls);
// }
