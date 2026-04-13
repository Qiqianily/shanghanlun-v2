use std::time::Duration;

use axum::{extract::Request, http::Response};
use tower_http::trace::{MakeSpan, OnRequest, OnResponse};
use tracing::Span;

/// Latency 延迟用时
struct Latency(Duration);

/// 重写了 display trait 如果是 ms 就返回，否则返回 μs
impl std::fmt::Display for Latency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{} ms", self.0.as_millis())
        } else {
            write!(f, "{} μs", self.0.as_micros())
        }
    }
}

/// Latency on response 响应延时
#[derive(Debug, Clone, Copy)]
pub struct LatencyOnResponse;

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        tracing::info!(
            latency = %Latency(latency),
            status = response.status().as_u16(),
            "Finished processing request"
        )
    }
}

#[derive(Clone)]
pub struct LogOnRequest;

impl<B> OnRequest<B> for LogOnRequest {
    fn on_request(&mut self, request: &Request<B>, _span: &Span) {
        tracing::info!("➡️ ✅ {} {}", request.method(), request.uri().path());
    }
}

// 自定义 MakeSpan：为每个请求创建带有关键字段的 span（非必须，但利于追踪）
#[derive(Clone)]
pub struct CustomMakeSpan;

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let method = request.method();
        let path = request.uri().path();
        let id = xid::new();
        tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
    }
}
