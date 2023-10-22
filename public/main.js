import init, { format_wasm } from "./pkg/paper_formatter.js";

function restore_config() {
  // ローカルストレージから設定を復元
  const storedConfig = localStorage.getItem("config");
  if (storedConfig) {
    const config = JSON.parse(storedConfig);
    apply_config(config);
  } else {
    to_default();
  }
}

function save_config(config) {
  // 設定をローカルストレージに保存
  localStorage.setItem("config", JSON.stringify(config));
}

function check_all() {
  const checkboxes = document.querySelectorAll('input[type="checkbox"]');
  for (let i = 0; i < checkboxes.length; i++) {
    checkboxes[i].checked = true;
  }
}

function uncheck_all() {
  const checkboxes = document.querySelectorAll('input[type="checkbox"]');
  for (let i = 0; i < checkboxes.length; i++) {
    checkboxes[i].checked = false;
  }
}

function apply_config(config) {
  Object.keys(config).forEach((key) => {
    const checkbox = document.getElementById(key);
    checkbox.checked = config[key];
  });
}

function to_default() {
  const config = {
    to_normal: true,
    nl_to_space: true,
    restore_word: true,
    ignore_enters: true,
    enter_with_end: true,
    split: true,
    quiet_mode: false,
  };
  apply_config(config);
}

function format() {
  init().then(() => {
    const text = document.getElementById("target").value;

    const config = {
      to_normal: document.getElementById("to_normal").checked,
      nl_to_space: document.getElementById("nl_to_space").checked,
      restore_word: document.getElementById("restore_word").checked,
      ignore_enters: document.getElementById("ignore_enters").checked,
      enter_with_end: document.getElementById("enter_with_end").checked,
      split: document.getElementById("split").checked,
      quiet_mode: document.getElementById("quiet_mode").checked,
    };

    const config_str = JSON.stringify(config);

    const res_arr = format_wasm(text, config_str);
    const res = res_arr.join("\n");

    document.getElementById("result").value = res;

    save_config(config);
  });
}

function translate() {
  init().then(() => {
    const text = document.getElementById("target").value;

    const config = {
      to_normal: document.getElementById("to_normal").checked,
      nl_to_space: document.getElementById("nl_to_space").checked,
      restore_word: document.getElementById("restore_word").checked,
      ignore_enters: document.getElementById("ignore_enters").checked,
      enter_with_end: document.getElementById("enter_with_end").checked,
      split: document.getElementById("split").checked,
      quiet_mode: document.getElementById("quiet_mode").checked,
    };

    const config_str = JSON.stringify(config);

    const res_arr = format_wasm(text, config_str);

    if (config.quiet_mode) {
      res_arr.forEach((res, i) => {
        const encoded = encodeURI(res);
        const url = "https://www.deepl.com/translator#en/ja/" + encoded;

        window.open(url, "_blank");
      });
    } else {
      let message = res_arr.length + "個のDeepLタブを表示します。";
      if (confirm(message)) {
        res_arr.forEach((res, i) => {
          const encoded = encodeURI(res);
          const url = "https://www.deepl.com/translator#en/ja/" + encoded;

          window.open(url, "_blank");
        });
      }
    }

    save_config(config);
  });
}

function reset() {
  document.getElementById("target").value = "";
  document.getElementById("result").value = "";
}

restore_config();

document.getElementById("format").addEventListener("click", format);
document.getElementById("check_all").addEventListener("click", check_all);
document.getElementById("uncheck_all").addEventListener("click", uncheck_all);
document.getElementById("default").addEventListener("click", to_default);
document.getElementById("reset").addEventListener("click", reset);
document.getElementById("translate").addEventListener("click", translate);
