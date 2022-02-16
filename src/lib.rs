use mpris::PlayerFinder;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn run() -> Result<(), &'static str> {
    let metadata = match match match PlayerFinder::new() {
        Ok(val) => val,
        Err(_) => return Err("Could not connect to D-Bus"),
    }
    .find_active()
    {
        Ok(val) => val,
        Err(_) => return Err("Could not find any players"),
    }
    .get_metadata()
    {
        Ok(val) => val,
        Err(_) => return Err("Could not get metadata"),
    };

    let media_string = format(
        match metadata.artists() {
            Some(val) => val,
            None => return Err("Couldn't get artists data"),
        }
        .join(" & ")
        .as_str(),
        match metadata.title() {
            Some(val) => val,
            None => return Err("Couldn't get title data"),
        },
    );

    if media_string.width() > 100 {
        println!("{}...", cut(&media_string[..], 97));
    } else {
        println!("{}", media_string);
    }

    Ok(())
}

fn format(a: &str, t: &str) -> String {
    match (a, t) {
        ("", "") => String::new(),
        ("", x) | (x, "") => x.to_string(),
        (a, t) => format!("{a} - {t}"),
    }
}

fn cut(s: &str, max_size: usize) -> String {
    let mut i = 0;
    let mut out = String::new();
    for c in s.chars() {
        if let Some(width) = c.width() {
            i += width;
            if i > max_size {
                break;
            }
            out.push(c);
        } else {
            // filters out control characters besides \x00
            unimplemented!();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats() {
        let a = "foo";
        let t = "bar";
        assert_eq!(format(a, t), "foo - bar");
        assert_eq!(format(a, ""), "foo");
        assert_eq!(format("", t), "bar");
    }

    #[test]
    fn cuts() {
        let teststr = "容ゆい製理ク連方館レうル球5焼きとなひ達庁ひる聞企すどに賀国せフすは広非ラツイシ座点セチ内投者歯没まる。";
        assert_eq!(cut(teststr, 100), "容ゆい製理ク連方館レうル球5焼きとなひ達庁ひる聞企すどに賀国せフすは広非ラツイシ座点セチ内投者歯没ま");
    }
}
