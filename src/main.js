const { invoke } = window.__TAURI__.core;

let InputEl;
let MsgEl;
let paletteContainer;


async function infer() {
  try {
    const colors = await invoke("fetch_palette_from_llm", { prompt: InputEl.value });
    MsgEl.textContent = "";
    renderPalette(colors);
  } catch (e) {
    MsgEl.textContent = `Error: ${e}`;
    paletteContainer.innerHTML = "";
  }
}

function renderPalette(colors) {
  paletteContainer.innerHTML = "";
  colors.forEach(hex => {
    const swatch = document.createElement("div");
    swatch.className = "swatch";
    swatch.style.backgroundColor = hex;
    swatch.title = hex.toUpperCase();
    swatch.addEventListener("click", () => {
      navigator.clipboard.writeText(hex.toUpperCase());
      const orig = swatch.title;
      swatch.title = "Copied!";
      setTimeout(() => { swatch.title = orig; }, 1500);
    });
    const label = document.createElement("span");
    label.className = "swatch-label";
    label.textContent = hex.toUpperCase();
    swatch.appendChild(label);
    paletteContainer.appendChild(swatch);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  InputEl = document.querySelector("#greet-input");
  MsgEl = document.querySelector("#greet-msg");
  paletteContainer = document.querySelector("#palette");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    infer();
  });
});
