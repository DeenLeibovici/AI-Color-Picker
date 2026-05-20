const { invoke } = window.__TAURI__.core;

let InputEl;
let MsgEl;
let circleColor;


async function infer() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  try {
    MsgEl.textContent = await invoke("fetch_color_from_llm", { prompt: InputEl.value });
    circleColor.style.backgroundColor = MsgEl.textContent;
  } catch (e) {
    MsgEl.textContent = `Error: ${e}`;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  InputEl = document.querySelector("#greet-input");
  MsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    infer();
  });

  circleColor = document.querySelector(".circle");
});
