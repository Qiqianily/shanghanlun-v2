use chrono::Utc;
use sha2::{Digest, Sha256};

fn main() -> anyhow::Result<()> {
    let mut hasher = Sha256::new();
    // let ip = "45.113.2.23";
    // let ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36";
    let ip = "unknown";
    let ua = "unknown";
    let bytes_str = format!("{}|{}", ip, ua);
    hasher.update(bytes_str.as_bytes());
    let hash = hasher.finalize();
    let hash_str = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    println!("Result: {}", hash_str);
    println!("len: {}", hash_str.len());
    let today_date = Utc::now().date_naive();
    println!("today:{}", today_date);
    Ok(())
}
