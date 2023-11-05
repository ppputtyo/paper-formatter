use paper_formatter::{format, Config};

fn compare(res: &[String], gold: &[&str]) {
    for (i, tmp) in res.iter().enumerate() {
        assert_eq!(tmp, &gold[i]);
    }
}

#[test]
fn test_simple() {
    let special_text = r"I am an
𝔸pple. You are a pine-
apple.";
    let gold = ["I am an Apple.\nYou are a pineapple."];

    let config = Config::default();
    let res = format(special_text, &config);

    compare(&res, &gold);
}

#[test]
fn test_question_exclamation() {
    let special_text = r"I am hoge? No, I am huga.";
    let gold = ["I am hoge?\nNo, I am huga."];

    let config = Config::default();

    let res = format(special_text, &config);

    compare(&res, &gold);
}
