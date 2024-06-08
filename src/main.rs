use num_bigint::BigUint;
use std::fs;
use std::time::Duration;
use std::{io, thread};
use chrono::{Timelike, Utc};

pub fn encrypt_pass(password: String) -> String {
    let e = 65537u32;
    let n = BigUint::parse_bytes(b"94dd2a8675fb779e6b9f7103698634cd400f27a154afa67af6166a43fc26417222a79506d34cacc7641946abda1785b7acf9910ad6a0978c91ec84d40b71d2891379af19ffb333e7517e390bd26ac312fe940c340466b4a5d4af1d65c3b5944078f96a1a51a5a53e4bc302818b7c9f63c4a1b07bd7d874cef1c3d4b2f5eb7871",16).unwrap();
    let c = BigUint::from_bytes_be(&password.as_bytes());
    let result = c.modpow(&BigUint::from(e), &n);
    format!("{:0>256}", result.to_str_radix(16))
}

pub fn extract<'a>(text: &'a str, prefix: &'a str, suffix: &'a str) -> io::Result<&'a str> {
    let left = text.find(prefix);
    let right = text.find(suffix);
    if let (Some(l), Some(r)) = (left, right) {
        if l + prefix.len() < r {
            return Ok(&text[l + prefix.len()..r]);
        }
    }
    Err(io::ErrorKind::InvalidData.into())
}

pub fn login(username: &str, password: &str) -> io::Result<()> {
    let resp = minreq::get("http://www.baidu.com")
        .with_timeout(10)
        .send()
        .map_err(|e| {
            println!("[Err] Ping Fail {}", e);
            io::ErrorKind::ConnectionRefused
        })?;
    let resp = resp.as_str().map_err(|e| {
        println!("[Err] Invalid Resp Format {}", e);
        io::ErrorKind::InvalidData
    })?;
    if !resp.contains("/eportal/index.jsp")
        && !resp.contains("<script>top.self.location.href='http://")
    {
        return Ok(());
    }
    let portal_ip = extract(
        resp,
        "<script>top.self.location.href='http://",
        "/eportal/index.jsp",
    )?;
    // println!("portal ip: {}", portal_ip);
    let mac = extract(resp, "mac=", "&t=")?;
    // println!("mac: {}", mac);
    let encrypt_pass = encrypt_pass(format!("{}>{}", password, mac));
    let query_string = extract(resp, "/eportal/index.jsp?", "'</script>\r\n")?;
    // println!("query_string: {}", query_string);
    let query_string = urlencoding::encode(query_string);
    let body = format!(
        "userId={}&password={}&service=&queryString={}&passwordEncrypt=true",
        username, encrypt_pass, query_string
    );
    let login_url = format!("http://{}/eportal/InterFace.do?method=login", portal_ip);
    let resp = minreq::post(login_url)
        .with_body(body)
        .with_header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .with_header("Accept", "*/*")
        .with_header("User-Agent", "hust-network-login")
        .with_timeout(10)
        .send()
        .map_err(|e| {
            println!("[Err] Portal Fail {}", e);
            io::ErrorKind::ConnectionRefused
        })?;
    let resp = resp.as_str().map_err(|e| {
        println!("[Err] Invalid Login Resp Format {}", e);
        io::ErrorKind::InvalidData
    })?;
    // println!("login resp: {}", resp);
    if resp.contains("success") {
        Ok(())
    } else {
        Err(io::ErrorKind::PermissionDenied.into())
    }
}

pub fn get_time() -> bool {
    let now = Utc::now();
    let hour = now.hour();
    let mut is_net_open = true;
    if hour > 15 && hour < 23 {
        is_net_open = false;
    }
    println!("[Info] UTC Hour: {} Work Status: {}", hour, is_net_open);
    return is_net_open;
}

fn main() {
    let args = std::env::args();
    if args.len() <= 1 {
        panic!("[Err] No Config File")
    }
    let path = args.last().unwrap();
    let s = String::from_utf8(fs::read(&path).unwrap()).unwrap();
    let mut lines = s.lines();
    let username = lines.next().unwrap().to_owned();
    let password = lines.next().unwrap().to_owned();
    loop {
        let is_work_time = get_time();
        if is_work_time == true {
            match login(&username, &password) {
                Ok(_) => {
                    println!("[Info] Login OK");
                    thread::sleep(Duration::from_secs(10));
                }
                Err(e) => {
                    println!("[Err] Login Fail {}", e);
                    thread::sleep(Duration::from_secs(5));
                }
            }
            // thread::sleep(Duration::from_secs(5));
        } else {
            println!("[Warn] Not Work Time");
            thread::sleep(Duration::from_secs(60));
        }
    }
}
