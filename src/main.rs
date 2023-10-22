use paper_formatter::{format, Config};

fn main() {
    let special_text = r"
Test. Te-

st.

Test.";
    let config = Config {
        to_normal: true,
        restore_word: true,
        nl_to_space: true,
        ignore_enters: false,
        enter_with_end: true,
        split: true,
        quiet_mode: true,
    };

    let res = format(special_text, &config);

    for tmp in res {
        dbg!(tmp.len());
        println!("{}", tmp);
    }
}
