use paper_formatter::{format, Config};

fn main() {
    let special_text = r"";
    let config = Config::new();

    let res = format(special_text, &config);

    for tmp in res {
        dbg!(tmp.len());
        println!("{}", tmp);
    }
}
