use chrono::Utc;

fn main() -> anyhow::Result<()> {
    let today_date = Utc::now().date_naive();
    let res = today_date - chrono::Duration::days(3);
    println!("today:{}", today_date);
    println!("yesterday:{}", res);

    Ok(())
}
