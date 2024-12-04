use analytics_server::pb::{
    analytics_event::EventType, app_exit_event::ExitCode, AnalyticsEvent, AppExitEvent,
    EventContext, SystemInfo,
};
use anyhow::Result;
use prost::Message;

#[tokio::main]
async fn main() -> Result<()> {
    let mut context = EventContext::default();
    context.client_id = "123".to_string();
    context.user_id = "123".to_string();
    context.ip = "123.123.123.123".to_string();
    context.user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".to_string();
    context.app_version = "1.0.0".to_string();
    context.client_ts = chrono::Utc::now().timestamp_millis();
    context.system = Some(SystemInfo {
        os: "macos".to_string(),
        arch: "x64".to_string(),
        locale: "zh-CN".to_string(),
        timezone: "Asia/Shanghai".to_string(),
    });

    let exit = AppExitEvent {
        exit_code: ExitCode::Success.into(),
    };
    let mut event = AnalyticsEvent::default();
    event.context = Some(context);
    event.event_type = Some(EventType::AppExit(exit));

    let client = reqwest::Client::new();
    let data = event.encode_to_vec();
    // write data to "../../fixtures/event.bin"
    std::fs::write("../../fixtures/event.bin", &data)?;
    // load data from "../../fixtures/event.bin"
    let data1 = std::fs::read("../../fixtures/event.bin")?;
    // parse data1 to event
    let event1 = AnalyticsEvent::decode(data1.as_slice())?;
    println!("{:?}", event1);
    let res = client
        .post("http://localhost:6690/api/event")
        .header("Content-Type", "application/protobuf")
        .body(data)
        .send()
        .await?;
    println!("{:?}", res.text().await?);
    Ok(())
}
