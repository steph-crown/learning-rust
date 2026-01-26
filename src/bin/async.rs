use std::time::{Duration, Instant};

use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
  let response = trpl::get(url).await;
  let response_text = response.text().await;
  Html::parse(&response_text)
    .select_first("title")
    .map(|title_element| title_element.inner_html())
}

fn main() {
  // trpl::run(async {
  //   let x = trpl::spawn_task(async {
  //     for i in 1..10 {
  //       println!("hi number {i} from the first task!");
  //       trpl::sleep(Duration::from_millis(500)).await;
  //     }
  //   });

  //   for i in 1..5 {
  //     println!("hi number {i} from the second task!");
  //     trpl::sleep(Duration::from_millis(500)).await;
  //   }
  //   let x = x.await;
  // });
  trpl::run(async {
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
      for _ in 1..1000 {
        trpl::sleep(one_ns).await;
      }
    }
    .await;
    let time = Instant::now() - start;
    println!(
      "'sleep' version finished after {} seconds.",
      time.as_secs_f32()
    );

    let start = Instant::now();
    async {
      for _ in 1..1000 {
        trpl::yield_now().await;
      }
    }
    .await;
    let time = Instant::now() - start;
    println!(
      "'yield' version finished after {} seconds.",
      time.as_secs_f32()
    );
  });
}
