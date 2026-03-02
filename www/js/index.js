// www/js/index.js
import init, {
  getsize,
  getpyform,
  connectpyform,
  getziform,
  connectziform,
  getstrokeform,
  connectstrokeform,
  listdic,
} from "../pkg/wasmzidian.js";

async function run() {
  const wasm = await init();

  // necessary because index.js will be loaded as a wasm module;
  // directly attaching printsize() to the button in index.html does not work:
  let b = document.getElementById("getsize");
  b.addEventListener("click", (ev) => printsize());

  let c = document.getElementById("listforpy");
  c.addEventListener("click", (ev) => printpyform());

  let d = document.getElementById("listforzi");
  d.addEventListener("click", (ev) => printziform());

  let e = document.getElementById("listforstrokes");
  e.addEventListener("click", (ev) => printstrokeform());

  let f = document.getElementById("listdic");
  f.addEventListener("click", (ev) => listdic());
}

run();

function printsize() {
  let container = document.getElementById("content");
  container.innerHTML = "Dictionary size : " + getsize() + " zi";
}

function printpyform() {
  let container = document.getElementById("content");
  container.innerHTML = getpyform();
  connectpyform(); // back to wasm for form submit callback
  document.getElementById("pinyin").focus(); // autofocus is not working, but this works
  let c = document.getElementById("cancel");
  c.addEventListener("click", (ev) => cancel());
}

function printziform() {
  let container = document.getElementById("content");
  container.innerHTML = getziform();
  connectziform(); // back to wasm for form submit callback
  document.getElementById("carac").focus(); // autofocus is not working, but this works
  let c = document.getElementById("cancel");
  c.addEventListener("click", (ev) => cancel());
}

function printstrokeform() {
  let container = document.getElementById("content");
  container.innerHTML = getstrokeform();
  connectstrokeform(); // back to wasm for form submit callback
  document.getElementById("stroke").focus(); // autofocus is not working, but this works
  let c = document.getElementById("cancel");
  c.addEventListener("click", (ev) => cancel());
}

function cancel() {
  let container = document.getElementById("content");
  container.innerHTML = "Form canceled.";
}
