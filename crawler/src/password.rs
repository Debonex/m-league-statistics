use std::time::{SystemTime, UNIX_EPOCH};

pub fn gen_password() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut t: i64 = now as i64 / 1000_i64 - 1577804400;
    t = sha(t, 2967816889);
    t = convert(t);
    t = sha(t, 4009073545);

    format!("xxxxxxxx{:04x}{:04x}", (t >> 16) & 65535, 65535 & t)
}

fn convert(e: i64) -> i64 {
    let mut e = ((e >> 1) & 1431655765) | ((1431655765 & e) << 1);
    e = ((e >> 2) & 858993459) | ((858993459 & e) << 2);
    e = ((e >> 4) & 252645135) | ((252645135 & e) << 4);
    e = ((e >> 8) & 16711935) | ((16711935 & e) << 8);
    (e >> 16) | (65535 & e) << 16
}

fn sha(e: i64, t: i64) -> i64 {
    let n = 65535 & e;
    let a = 65535 & t;
    let o = n * a;
    (((((o >> 16) & 65535)
        + ((((e >> 16) & 65535) * a) & 65535)
        + ((n * ((t >> 16) & 65535)) & 65535))
        & 65535)
        << 16)
        | (65535 & o)
}
