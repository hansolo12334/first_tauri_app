const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

let usernameInputEl;
let usernameMsgEl;


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function greet_to_user() {
  usernameMsgEl.textContent = await invoke("greet_to_user", { name: usernameInputEl.value });
}

async function openNewWindow() {
  console.log("openWindow!");
  try {
    await invoke("open_new_page");
  } catch (error) {
    console.error("打开窗口失败", error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

window.addEventListener("DOMContentLoaded", () => {
  usernameInputEl = document.querySelector("#username-input");
  usernameMsgEl = document.querySelector("#username-msg");
  document.querySelector("#get-user-from").addEventListener("submit", (e) => {
    e.preventDefault();
    greet_to_user();
  });
});

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector('.openNewWindow-button').addEventListener('click', openNewWindow);
});