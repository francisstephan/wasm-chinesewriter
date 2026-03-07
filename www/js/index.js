// www/js/index.js
import init, {
  getsize,
  getpyform,
  connectpyform,
  getziform,
  connectziform,
  getstrokeform,
  connectstrokeform,
  whz,
  connectwhzform,
  listdic,
} from "../pkg/wasmzidian.js";

async function run() {
  const wasm = await init();

  // necessary because index.js will be loaded as a wasm module;
  // directly attaching printsize() to the button in index.html does not work:
  document
    .getElementById("getsize")
    .addEventListener("click", (ev) => printsize());
  document
    .getElementById("listforpy")
    .addEventListener("click", (ev) => printpyform());
  document
    .getElementById("listforzi")
    .addEventListener("click", (ev) => printziform());
  document
    .getElementById("listforstrokes")
    .addEventListener("click", (ev) => printstrokeform());
  document
    .getElementById("listdic")
    .addEventListener("click", (ev) => printdic());
  document
    .getElementById("writehanzi")
    .addEventListener("click", (ev) => printwhz());
}

run();

function printsize() {
  document.body.removeEventListener("keydown", esckey); // esckey defined in chinesewriter.js
  let container = document.getElementById("content");
  container.innerHTML = "Dictionary size : " + getsize() + " zi";
}

function printpyform() {
  hidemenu("lists");
  let container = document.getElementById("content");
  container.innerHTML = getpyform();
  connectpyform(); // back to wasm for form submit callback
  document.getElementById("pinyin").focus(); // autofocus is not working, but this works
  document.getElementById("cancel").addEventListener("click", cancel);
  document.body.addEventListener("keydown", esckey); // esckey defined in chinesewriter.js
}

function printziform() {
  hidemenu("lists");
  let container = document.getElementById("content");
  container.innerHTML = getziform();
  connectziform(); // back to wasm for form submit callback
  document.getElementById("carac").focus(); // autofocus is not working, but this works
  let c = document.getElementById("cancel");
  c.addEventListener("click", (ev) => cancel());
  document.body.addEventListener("keydown", esckey); // esckey defined in chinesewriter.js
}

function printstrokeform() {
  hidemenu("lists");
  let container = document.getElementById("content");
  container.innerHTML = getstrokeform();
  connectstrokeform(); // back to wasm for form submit callback
  document.getElementById("stroke").focus(); // autofocus is not working, but this works
  let c = document.getElementById("cancel");
  c.addEventListener("click", (ev) => cancel());
  document.body.addEventListener("keydown", esckey); // esckey defined in chinesewriter.js
}

function printdic() {
  hidemenu("lists");
  document.body.removeEventListener("keydown", esckey); // esckey defined in chinesewriter.js
  listdic();
}

function printwhz() {
  let container = document.getElementById("content");
  container.innerHTML = whz();
  connectwhzform(); // back to wasm for form submit callback
  document.getElementById("pinyin").focus(); // autofocus is not working, but this works
  document.body.removeEventListener("keydown", esckey); // esckey defined in chinesewriter.js
}
