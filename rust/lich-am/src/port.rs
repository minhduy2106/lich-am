//! The Rust port of `lunar.js`: Vietnamese lunar calendar (UTC+7), Hồ Ngọc Đức's algorithm,
//! plus .ics export. Arithmetic stays in f64 in the same order as the JS so results match bit for bit.

#![forbid(unsafe_code)]

use serde_json::{Map, Value, json};
use std::f64::consts::PI;

const TZ: f64 = 7.0;

fn num(v: &Value, name: &str) -> Result<f64, String> {
    v.as_f64()
        .ok_or_else(|| format!("TypeError: {name} must be a number"))
}

/// A JS number as JSON: integers without a fractional part, like `JSON.stringify`.
fn js_num(x: f64) -> Value {
    if x.fract() == 0.0 && x.abs() < 9.0e15 {
        json!(x as i64)
    } else {
        json!(x)
    }
}

fn jd_from(dd: f64, mm: f64, yy: f64) -> f64 {
    let a = ((14.0 - mm) / 12.0).floor();
    let y = yy + 4800.0 - a;
    let m = mm + 12.0 * a - 3.0;
    let jd = dd + ((153.0 * m + 2.0) / 5.0).floor() + 365.0 * y + (y / 4.0).floor()
        - (y / 100.0).floor()
        + (y / 400.0).floor()
        - 32045.0;
    if jd < 2299161.0 {
        dd + ((153.0 * m + 2.0) / 5.0).floor() + 365.0 * y + (y / 4.0).floor() - 32083.0
    } else {
        jd
    }
}

fn jd_to(jd: f64) -> [f64; 3] {
    let (b, c) = if jd > 2299160.0 {
        let a = jd + 32044.0;
        let b = ((4.0 * a + 3.0) / 146097.0).floor();
        (b, a - ((b * 146097.0) / 4.0).floor())
    } else {
        (0.0, jd + 32082.0)
    };
    let d = ((4.0 * c + 3.0) / 1461.0).floor();
    let e = c - ((1461.0 * d) / 4.0).floor();
    let m = ((5.0 * e + 2.0) / 153.0).floor();
    let day = e - ((153.0 * m + 2.0) / 5.0).floor() + 1.0;
    let month = m + 3.0 - 12.0 * (m / 10.0).floor();
    let year = b * 100.0 + d - 4800.0 + (m / 10.0).floor();
    [day, month, year]
}

fn new_moon_day(k: f64, tz: f64) -> f64 {
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let dr = PI / 180.0;
    let mut jd1 = 2415020.75933 + 29.53058868 * k + 0.0001178 * t2 - 0.000000155 * t3;
    jd1 += 0.00033 * ((166.56 + 132.87 * t - 0.009173 * t2) * dr).sin();
    let m = 359.2242 + 29.10535608 * k - 0.0000333 * t2 - 0.00000347 * t3;
    let mpr = 306.0253 + 385.81691806 * k + 0.0107306 * t2 + 0.00001236 * t3;
    let f = 21.2964 + 390.67050646 * k - 0.0016528 * t2 - 0.00000239 * t3;
    let mut c1 = (0.1734 - 0.000393 * t) * (m * dr).sin() + 0.0021 * (2.0 * dr * m).sin();
    c1 = c1 - 0.4068 * (mpr * dr).sin() + 0.0161 * (dr * 2.0 * mpr).sin();
    c1 -= 0.0004 * (dr * 3.0 * mpr).sin();
    c1 = c1 + 0.0104 * (dr * 2.0 * f).sin() - 0.0051 * (dr * (m + mpr)).sin();
    c1 = c1 - 0.0074 * (dr * (m - mpr)).sin() + 0.0004 * (dr * (2.0 * f + m)).sin();
    c1 = c1 - 0.0004 * (dr * (2.0 * f - m)).sin() - 0.0006 * (dr * (2.0 * f + mpr)).sin();
    c1 = c1 + 0.0010 * (dr * (2.0 * f - mpr)).sin() + 0.0005 * (dr * (2.0 * mpr + m)).sin();
    let deltat = if t < -11.0 {
        0.001 + 0.000839 * t + 0.0002261 * t2 - 0.00000845 * t3 - 0.000000081 * t * t3
    } else {
        -0.000278 + 0.000265 * t + 0.000262 * t2
    };
    (jd1 + c1 - deltat + 0.5 + tz / 24.0).floor()
}

fn sun_longitude(jdn: f64, tz: f64) -> f64 {
    let t = (jdn - 2451545.5 - tz / 24.0) / 36525.0;
    let t2 = t * t;
    let dr = PI / 180.0;
    let m = 357.5291 + 35999.0503 * t - 0.0001559 * t2 - 0.00000048 * t * t2;
    let l0 = 280.46645 + 36000.76983 * t + 0.0003032 * t2;
    let mut dl = (1.9146 - 0.004817 * t - 0.000014 * t2) * (dr * m).sin();
    dl += (0.019993 - 0.000101 * t) * (dr * 2.0 * m).sin() + 0.00029 * (dr * 3.0 * m).sin();
    let mut l = (l0 + dl) * dr;
    l -= PI * 2.0 * (l / (PI * 2.0)).floor();
    ((l / PI) * 6.0).floor()
}

fn lunar_month11(yy: f64, tz: f64) -> f64 {
    let off = jd_from(31.0, 12.0, yy) - 2415021.0;
    let k = (off / 29.530588853).floor();
    let nm = new_moon_day(k, tz);
    if sun_longitude(nm, tz) >= 9.0 {
        new_moon_day(k - 1.0, tz)
    } else {
        nm
    }
}

fn leap_month_offset(a11: f64, tz: f64) -> f64 {
    let k = ((a11 - 2415021.076998695) / 29.530588853 + 0.5).floor();
    let mut i = 1.0;
    let mut arc = sun_longitude(new_moon_day(k + i, tz), tz);
    loop {
        let last = arc;
        i += 1.0;
        arc = sun_longitude(new_moon_day(k + i, tz), tz);
        if arc == last || i >= 14.0 {
            break;
        }
    }
    i - 1.0
}

/// [day, month, lunar year, leap (0/1)].
fn solar_to_lunar(dd: f64, mm: f64, yy: f64, tz: f64) -> [f64; 4] {
    let day_number = jd_from(dd, mm, yy);
    let k = ((day_number - 2415021.076998695) / 29.530588853).floor();
    let mut month_start = new_moon_day(k + 1.0, tz);
    if month_start > day_number {
        month_start = new_moon_day(k, tz);
    }
    let mut a11 = lunar_month11(yy, tz);
    let mut b11 = a11;
    let mut lunar_year;
    if a11 >= month_start {
        lunar_year = yy;
        a11 = lunar_month11(yy - 1.0, tz);
    } else {
        lunar_year = yy + 1.0;
        b11 = lunar_month11(yy + 1.0, tz);
    }
    let lunar_day = day_number - month_start + 1.0;
    let diff = ((month_start - a11) / 29.0).floor();
    let mut lunar_leap = 0.0;
    let mut lunar_month = diff + 11.0;
    if b11 - a11 > 365.0 {
        let leap_month_diff = leap_month_offset(a11, tz);
        if diff >= leap_month_diff {
            lunar_month = diff + 10.0;
            if diff == leap_month_diff {
                lunar_leap = 1.0;
            }
        }
    }
    if lunar_month > 12.0 {
        lunar_month -= 12.0;
    }
    if lunar_month >= 11.0 && diff < 4.0 {
        lunar_year -= 1.0;
    }
    [lunar_day, lunar_month, lunar_year, lunar_leap]
}

fn lunar_of_jd(jd: f64) -> [f64; 4] {
    let [d, m, y] = jd_to(jd);
    solar_to_lunar(d, m, y, TZ)
}

/// JS truthiness for the option flags.
fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64().is_some_and(|x| x != 0.0 && !x.is_nan()),
        Some(Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

/// `${v}` for the values a name can hold.
fn js_text(v: Option<&Value>) -> String {
    match v {
        None => "undefined".into(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n
            .as_f64()
            .map_or_else(|| n.to_string(), |x| js_num(x).to_string()),
        Some(other) => other.to_string(),
    }
}

fn solar_json([d, m, y]: [f64; 3]) -> Value {
    json!([js_num(d), js_num(m), js_num(y)])
}

/// Port of `jdFromDate` (`lunar.js:4`).
pub fn jd_from_date(dd: Value, mm: Value, yy: Value) -> Result<Value, String> {
    Ok(js_num(jd_from(
        num(&dd, "dd")?,
        num(&mm, "mm")?,
        num(&yy, "yy")?,
    )))
}

/// Port of `jdToDate` (`lunar.js:13`).
pub fn jd_to_date(jd: Value) -> Result<Value, String> {
    Ok(solar_json(jd_to(num(&jd, "jd")?)))
}

/// Port of `convertSolar2Lunar` (`lunar.js:83`).
pub fn convert_solar2_lunar(
    dd: Value,
    mm: Value,
    yy: Value,
    tz: Option<Value>,
) -> Result<Value, String> {
    let tz = match tz {
        None => TZ,
        Some(v) if crate::js::is_undefined(&v) => TZ,
        // JS: tham số mặc định không áp cho null, và null / 24 = 0.
        Some(Value::Null) => 0.0,
        Some(v) => num(&v, "tz")?,
    };
    let r = solar_to_lunar(num(&dd, "dd")?, num(&mm, "mm")?, num(&yy, "yy")?, tz);
    Ok(Value::Array(r.into_iter().map(js_num).collect()))
}

/// Port of `buildEvents` (`lunar.js:117`).
pub fn build_events(
    from_jd: Value,
    days: Value,
    gios: Value,
    opts: Value,
) -> Result<Value, String> {
    let from = num(&from_jd, "fromJd")?;
    let days = num(&days, "days")?;
    let gios = gios.as_array().cloned().unwrap_or_default();
    let mung1 = truthy(opts.get("mung1"));
    let ram = truthy(opts.get("ram"));
    let mut events = Vec::new();
    let push = |events: &mut Vec<Value>, solar: [f64; 3], kind: &str, title: String| {
        let mut e = Map::new();
        e.insert("solar".into(), solar_json(solar));
        e.insert("kind".into(), json!(kind));
        e.insert("title".into(), json!(title));
        events.push(Value::Object(e));
    };
    let mut next = lunar_of_jd(from);
    let mut jd = from;
    while jd < from + days {
        let [ld, lm, _, leap] = next;
        next = lunar_of_jd(jd + 1.0);
        let solar = jd_to(jd);
        let thang = format!(
            "tháng {}{}",
            js_num(lm),
            if leap != 0.0 { " nhuận" } else { "" }
        );
        if mung1 && ld == 1.0 {
            push(&mut events, solar, "le", format!("Mùng 1 {thang} (âm)"));
        }
        if ram && ld == 15.0 {
            push(&mut events, solar, "le", format!("Rằm {thang} (âm)"));
        }
        let last_day_of_month = next[0] == 1.0;
        for g in &gios {
            let day = g.get("day").and_then(Value::as_f64);
            let month = g.get("month").and_then(Value::as_f64);
            // Giỗ cúng vào tháng thường; tháng thiếu thì giỗ ngày 30 dời về ngày 29.
            let hit = day == Some(ld) || (last_day_of_month && day.is_some_and(|d| d > ld));
            if leap == 0.0 && month == Some(lm) && hit {
                let title = format!(
                    "Giỗ {} ({}/{} âm)",
                    js_text(g.get("name")),
                    js_text(g.get("day")),
                    js_text(g.get("month"))
                );
                push(&mut events, solar, "gio", title);
            }
        }
        jd += 1.0;
    }
    Ok(Value::Array(events))
}

fn hash(s: &str) -> String {
    let mut h: u64 = 0;
    for c in s.chars() {
        h = (h * 31 + u64::from(c)) % (1 << 32);
    }
    if h == 0 {
        return "0".into();
    }
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    while h > 0 {
        out.push(digits[(h % 36) as usize]);
        h /= 36;
    }
    out.reverse();
    String::from_utf8_lossy(&out).into_owned()
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' | ';' | ',' => {
                out.push('\\');
                out.push(c);
            }
            '\r' if chars.peek() == Some(&'\n') => {
                chars.next();
                out.push(' ');
            }
            '\n' => out.push(' '),
            _ => out.push(c),
        }
    }
    out
}

fn ymd([d, m, y]: [f64; 3]) -> String {
    let p = |n: f64| format!("{:0>2}", js_num(n).to_string());
    format!("{}{}{}", js_num(y), p(m), p(d))
}

fn solar_of(e: &Value) -> Result<[f64; 3], String> {
    let s = e
        .get("solar")
        .and_then(Value::as_array)
        .filter(|a| a.len() == 3)
        .ok_or("TypeError: event.solar must be [day, month, year]")?;
    Ok([
        num(&s[0], "day")?,
        num(&s[1], "month")?,
        num(&s[2], "year")?,
    ])
}

/// Port of `toIcs` (`lunar.js:146`).
pub fn to_ics(events: Value, now: Value) -> Result<Value, String> {
    let now = now
        .as_str()
        .ok_or("TypeError: now.replace is not a function")?;
    let stamp: String = now
        .chars()
        .filter(|c| *c != '-' && *c != ':')
        .take(15)
        .collect();
    let stamp = format!("{stamp}Z");
    let mut lines: Vec<String> = [
        "BEGIN:VCALENDAR",
        "VERSION:2.0",
        "PRODID:-//lich-am//VI",
        "CALSCALE:GREGORIAN",
        "X-WR-CALNAME:Lịch âm",
    ]
    .map(String::from)
    .to_vec();
    for e in events.as_array().map(Vec::as_slice).unwrap_or_default() {
        let solar = solar_of(e)?;
        let title = js_text(e.get("title"));
        let start = ymd(solar);
        let [d, m, y] = solar;
        lines.push("BEGIN:VEVENT".into());
        lines.push(format!("UID:{start}-{}@lich-am", hash(&title)));
        lines.push(format!("DTSTAMP:{stamp}"));
        lines.push(format!("DTSTART;VALUE=DATE:{start}"));
        lines.push(format!(
            "DTEND;VALUE=DATE:{}",
            ymd(jd_to(jd_from(d, m, y) + 1.0))
        ));
        lines.push(format!("SUMMARY:{}", esc(&title)));
        lines.push("TRANSP:TRANSPARENT".into());
        // Sự kiện cả ngày bắt đầu lúc 0h: -PT6H = 18h hôm trước, -PT63H = 9h sáng 3 ngày trước.
        let alarms: &[&str] = if e.get("kind").and_then(Value::as_str) == Some("gio") {
            &["-PT63H", "-PT6H"]
        } else {
            &["-PT6H"]
        };
        for t in alarms {
            lines.push("BEGIN:VALARM".into());
            lines.push("ACTION:DISPLAY".into());
            lines.push(format!("DESCRIPTION:{}", esc(&title)));
            lines.push(format!("TRIGGER:{t}"));
            lines.push("END:VALARM".into());
        }
        lines.push("END:VEVENT".into());
    }
    lines.push("END:VCALENDAR".into());
    Ok(json!(lines.join("\r\n") + "\r\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tet_and_leap_months() {
        assert_eq!(
            solar_to_lunar(29.0, 1.0, 2025.0, TZ),
            [1.0, 1.0, 2025.0, 0.0]
        );
        assert_eq!(
            solar_to_lunar(21.0, 1.0, 1985.0, TZ),
            [1.0, 1.0, 1985.0, 0.0]
        );
        assert_eq!(
            solar_to_lunar(25.0, 7.0, 2025.0, TZ),
            [1.0, 6.0, 2025.0, 1.0]
        );
    }

    #[test]
    fn gio_30_in_short_month_lands_before_tet() {
        let from = jd_from(29.0, 1.0, 2025.0);
        let days = jd_from(17.0, 2.0, 2026.0) - from;
        let gios = json!([{ "name": "ông nội", "day": 30, "month": 12 }]);
        let ev = build_events(json!(from), json!(days), gios, json!({})).unwrap_or_default();
        assert_eq!(
            ev,
            json!([{ "solar": [16, 2, 2026], "kind": "gio", "title": "Giỗ ông nội (30/12 âm)" }])
        );
    }

    #[test]
    fn escapes_and_hash() {
        assert_eq!(esc("a;b,c\\d\r\ne\nf\rg"), "a\\;b\\,c\\\\d e f\rg");
        assert_eq!(hash(""), "0");
    }
}
