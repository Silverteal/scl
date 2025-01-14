//! 获取模组中文名称的模块

/// 获取模组中文名称，如果没有则为空字符串，名称来自 MCMOD - Minecraft 模组中文百科
pub async fn get_mod_cname(modid: &str) -> String {
    // https://gitee.com/SteveXMH/scl-data/raw/master/mcmod/cname/chisel
    let modid = base64::encode_config(&modid, base64::URL_SAFE);
    if let Ok(mut resp) = crate::http::get(format!(
        "https://gitee.com/SteveXMH/scl-data/raw/master/mcmod/cname/{}",
        modid
    ))
    .await
    {
        if resp.status().is_success() {
            match resp.body_string().await {
                Ok(cname) => cname,
                _ => String::new(),
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}
