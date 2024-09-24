use reqwest::Error;
use std::time::Instant;
use tokio::time::{sleep, Duration};

async fn monitor_website(url: &str) -> Result<(), Error> {
    // 시작 시간을 기록
    let start = Instant::now();

    // HTTP GET 요청을 보내고 응답을 기다림
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => {
            // 응답 상태 코드
            let status = res.status();
            // 응답 시간을 계산
            let duration = start.elapsed();
            println!(
                "URL: {} - Status: {} - Response Time: {:.2?}",
                url, status, duration
            );
        }
        Err(e) => {
            // 요청에 실패했을 경우 오류 출력
            println!("Failed to reach {}: {}", url, e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // 모니터링할 URL 리스트
    let websites = vec!["https://www.example.com", "https://www.rust-lang.org"];

    // 주기적으로 웹사이트 상태를 확인
    loop {
        for website in &websites {
            // 각 웹사이트 모니터링
            if let Err(e) = monitor_website(website).await {
                println!("Error monitoring {}: {}", website, e);
            }
        }

        // 10초 대기 후 다시 모니터링
        sleep(Duration::from_secs(10)).await;
        println!("--------------------------------------------");
    }
}
