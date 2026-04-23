use sha2::{Digest, Sha256};

use crate::response::ApiResult;

/// 从请求头中提取客户端真实 IP
/// 优先使用 X-Real-IP（由反向代理设置），其次取 X-Forwarded-For 的第一个 IP
pub fn get_real_ip(headers: &axum::http::HeaderMap) -> ApiResult<String> {
    // 尝试 X-Real-IP
    if let Some(ip) = headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .filter(|ip| !ip.is_empty())
    {
        return Ok(ip.to_string());
    }
    // 尝试 X-Forwarded-For 取第一个 IP
    if let Some(forwarded) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        // 取第一个非空的 IP
        if let Some(first_ip) = forwarded.split(',').next().map(|s| s.trim())
            && !first_ip.is_empty()
        {
            return Ok(first_ip.to_string());
        }
    }
    // 如果以上获取都失败了，就返回 unknown
    Ok("unknown".to_string())
}

/// 获取 User-Agent 字符串（用于辅助标识）
pub fn get_user_agent(headers: &axum::http::HeaderMap) -> ApiResult<String> {
    if let Some(agent) = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .filter(|agent| !agent.is_empty())
    {
        return Ok(agent.to_string());
    }
    Ok("unknown".to_string())
}

/// 优先使用客户端生成的 ID 如果没有获取到，使用 IP 和 User-Agent 进行哈希
pub fn get_visitor_id(headers: &axum::http::HeaderMap) -> ApiResult<String> {
    // 优先使用客户端生成的 ID
    if let Some(client_id) = headers
        .get("x-client-id")
        .and_then(|v| v.to_str().ok())
        .filter(|id| !id.is_empty())
    {
        return Ok(client_id.to_string());
    }
    // 如果没有获取到客户端 ID，使用 IP 和 User-Agent 进行哈希
    // 获取 ip
    let ip = get_real_ip(headers)?;
    // 获取 User-Agent
    let agent = get_user_agent(headers)?;
    let raw = format!("{}|{}", ip, agent);
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    let hash = hasher.finalize();
    // 转成 string
    let hash_str = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    Ok(hash_str)
}
