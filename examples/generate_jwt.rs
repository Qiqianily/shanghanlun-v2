use shanghanlun::middlewares::auth::{
    identity::Identity, jwt::get_default_jwt, principal::Principal,
};

fn main() -> anyhow::Result<()> {
    let principal = Principal {
        id: 1,
        username: "Qiqianily".to_string(),
        identity: Identity::Admin,
    };
    let jwt = get_default_jwt();
    let token = jwt.encode(principal)?;
    // 输出 token
    println!("token: {token}");
    // 解码 token
    let role = jwt.decode(&token)?;
    println!("role: {:?}", role);
    Ok(())
}
