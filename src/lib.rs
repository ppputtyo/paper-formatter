use itertools::Itertools;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use unic_normal::StrNormalForm;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    to_normal: bool,
    nl_to_space: bool,
    restore_word: bool,
    ignore_enters: bool,
    enter_with_end: bool,
    split: bool,
    quiet_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            to_normal: true,
            nl_to_space: true,
            restore_word: true,
            ignore_enters: true,
            enter_with_end: true,
            split: true,
            quiet_mode: false,
        }
    }

    /// Json 文字列から Config 構造体を生成する
    pub fn from_json_str(json_str: &str) -> Config {
        serde_json::from_str(json_str).unwrap()
    }
}

fn split_at_multiple<'a>(input: &'a str, positions: &'a [usize]) -> Vec<&'a str> {
    if positions.is_empty() {
        return vec![input];
    }

    let mut res = vec![];

    for (i, pos) in positions.iter().enumerate() {
        if i == 0 {
            res.push(&input[0..*pos]);
        } else {
            res.push(&input[positions[i - 1]..*pos]);
        }
    }

    res.push(&input[positions[positions.len() - 1]..]);

    res
}

pub fn format(text: &str, config: &Config) -> Vec<String> {
    let mut text = text.to_string();

    let mut res = vec![];

    if config.to_normal {
        text = text.nfkc().collect::<String>();
    }

    if config.restore_word {
        let re = if config.ignore_enters {
            Regex::new(r"-\n").unwrap()
        } else {
            Regex::new(r"-\n+").unwrap()
        };

        text = re.replace_all(&text, "").to_string();
    }

    if config.nl_to_space {
        let re = if config.ignore_enters {
            Regex::new(r"\n").unwrap()
        } else {
            Regex::new(r"\n+").unwrap()
        };

        text = re.replace_all(&text, " ").to_string();
    }

    if config.enter_with_end {
        let re = Regex::new(r"\.\s*[A-Z]").unwrap();
        let space = Regex::new(r"\s").unwrap();
        text = re
            .replace_all(&text, |caps: &Captures| {
                format!("{}", space.replace(&caps[0], "\n"))
            })
            .to_string()
    }

    if config.split {
        let re = Regex::new(r"\.\s*[A-Z]").unwrap();

        let phrases_starts_points = re.find_iter(&text).map(|x| x.end() - 1).collect_vec();

        let phrases = split_at_multiple(&text, &phrases_starts_points);

        res.push(String::new());

        for phrase in phrases {
            if res.last().unwrap().len() + phrase.len() > 5000 {
                res.push(phrase.to_string());
            } else {
                res.last_mut().unwrap().push_str(phrase);
            }
        }
    } else {
        res.push(text);
    }

    res
}

#[wasm_bindgen]
pub fn format_wasm(text: &str, config_json_str: &str) -> js_sys::Array {
    let config = Config::from_json_str(config_json_str);

    let result = format(text, &config);

    let js_array = js_sys::Array::new();

    for tmp in result {
        js_array.push(&JsValue::from_str(&tmp));
    }

    js_array
}
