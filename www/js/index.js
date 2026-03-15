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
  getparseform,
  connectparseform,
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
  document
    .getElementById("parsestring")
    .addEventListener("click", (ev) => printparseform());
  minusesc_plusshort(); // initially:
  // remove esc key event listener
  // add p,z and s shortcut keys event listener
  // (see https://github.com/francisstephan/wasm-chinesewriter/blob/main/README.md )
  // defined in chinesewriter.js
}

run();

function printsize() {
  minusesc_plusshort();
  let container = document.getElementById("content");
  container.innerHTML = getsize();
}

function printpyform() {
  hidemenu("lists");
  let container = document.getElementById("content");
  container.innerHTML = getpyform();
  connectpyform(); // back to wasm for form submit callback
  document.getElementById("pinyin").focus(); // autofocus is not working, but this works
  document.getElementById("cancel").addEventListener("click", cancel);
  plusesc_minusshort();
}

function printziform() {
  hidemenu("lists");
  let container = document.getElementById("content");
  container.innerHTML = getziform();
  connectziform(); // back to wasm for form submit callback
  document.getElementById("carac").focus(); // autofocus is not working, but this works
  document.getElementById("cancel").addEventListener("click", cancel);
  plusesc_minusshort();
}

function printstrokeform() {
  hidemenu("lists");
  let container = document.getElementById("content");
  container.innerHTML = getstrokeform();
  connectstrokeform(); // back to wasm for form submit callback
  document.getElementById("stroke").focus(); // autofocus is not working, but this works
  document.getElementById("cancel").addEventListener("click", cancel);
  plusesc_minusshort();
}

function printdic() {
  hidemenu("lists");
  minusesc_plusshort();
  listdic(); // in wasm, cf lib.rs line 38
}

function printwhz() {
  let container = document.getElementById("content");
  container.innerHTML = whz();
  connectwhzform(); // back to wasm for form submit callback
  document.getElementById("pinyin").focus(); // autofocus is not working, but this works
  document.body.removeEventListener("keydown", esckey); // esckey defined in chinesewriter.js
  document.body.removeEventListener("keydown", shortkey);
}

function printparseform() {
  let container = document.getElementById("content");
  container.innerHTML = getparseform();
  connectparseform(); // back to wasm for form submit callback
  document.getElementById("zistr").focus(); // autofocus is not working, but this works
  document.getElementById("cancel").addEventListener("click", cancel);
  plusesc_minusshort();
}

function plusesc_minusshort() {
  document.body.addEventListener("keydown", esckey); // esckey defined in chinesewriter.js
  document.body.removeEventListener("keydown", shortkey);
  // console.log("esc activated");
}
