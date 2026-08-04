function convertToZi() {
  document.body.removeEventListener("keydown", esckey);
  var pyentree = document.getElementById("pinyin").value;
  var lon = pyentree.length;
  if (lon == 0) return;
  var dernier = pyentree.charAt(lon - 1);
  if ("01234/ ".includes(dernier)) {
    if (dernier == "/" || dernier == " ") {
      pyentree = pyentree.substring(0, lon - 1);
      document.getElementById("pinyin").value = pyentree;
    }
    // https://www.webdevtutor.net/blog/javascript-button-click-programmatically
    const button = document.getElementById("subpy");
    button.click();
  }
}

function add(zi) {
  var s = document.getElementById("zistring");
  s.value = s.value + zi;
  entree = document.getElementById("zilist"); // reset list of displayed zi buttons
  if (entree != null) entree.innerHTML = "";
  document.getElementById("pinyin").value = "";
  document.getElementById("pinyin").focus();
}

function copyTextToClipboard() {
  // source : https://stackoverflow.com/questions/400212/how-do-i-copy-to-the-clipboard-in-javascript
  var text = document.getElementById("zistring").value;
  navigator.clipboard.writeText(text).then(
    function () {
      console.log("Async: Copied " + text + "to clipboard!");
    },
    function (err) {
      console.error("Async: Could not copy text: ", err);
    },
  );
  document.getElementById("pinyin").focus();
}

function lookup(text) {
  var url =
    "https://translate.google.com/?sl=auto&tl=en&text=" +
    text +
    "&op=translate";
  window.open(url);
  var elem = document.getElementById("pinyin");
  if (elem) elem.focus();
}

function lookupWrittenChinese(text) {
  var url;
  if (text.length > 1) {
    url = "https://hanzii.net/search/word/" + text + "?hl=en";
  }
  else {
    url = "https://hanzii.net/search/kanji/" + text + "?hl=en";
  }
  window.open(url);
  var elem = document.getElementById("pinyin");
  if (elem) elem.focus();
}

function reset() {
  var entree = document.getElementById("zistring");
  if (entree != null) entree.value = "";
  entree = document.getElementById("zilist");
  if (entree != null) entree.innerHTML = "";
  document.getElementById("pinyin").value = "";
  document.getElementById("pinyin").focus();
}

function cancel() {
  // called from esckey (below) or from cancel button
  let container = document.getElementById("content");
  container.innerHTML = "Form canceled.";
  minusesc_plusshort();
}

function esckey(e) {
  // esckey : Event listener to cancel form with Esc key
  if (e.keyCode == 27) cancel();
}

function hidemenu(menu) {
  document.getElementById(menu).style.visibility = "hidden";
  document.getElementById("getlists").style.backgroundColor = "darkblue";
  document.getElementById("getlists").style.border = "2px solid darkblue";
}
function enablemenu(menu) {
  document.getElementById(menu).style.visibility = "visible";
  document.getElementById("getlists").style.backgroundColor = "green";
  document.getElementById("getlists").style.border = "2px solid palegreen";
}

function shortkey(e) {
  if (e.keyCode == 80) {
    // key p : pinyin => zi
    e.preventDefault(); // prevent transmission of p to input field
    document.getElementById("listforpy").click(); // https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click
  }
  if (e.keyCode == 90) {
    // key z : zi => pinyin
    e.preventDefault(); // prevent transmission of z to input field
    document.getElementById("listforzi").click();
  }
  if (e.keyCode == 83) {
    // key s : string parser
    e.preventDefault(); // prevent transmission of s to input field
    document.getElementById("parsestring").click();
  }
  if (e.keyCode == 87) {
    // key w : chinese writer
    e.preventDefault(); // prevent transmission of w to input field
    document.getElementById("writehanzi").click();
  }
}
function minusesc_plusshort() {
  // cannot be called from wasm if located in index.js,
  // because of directory structure: the index.js module is not reachable from wasm,
  // either at compile time if specified #[wasm_bindgen(module="../js/index.js")] extern "C" ...
  //       (which would be okay at runtime)
  // or a runtime if specified #[wasm_bindgen(module="/www/js/index.js")] extern "C" ...
  //       (which is ok at compile time ...)
  document.body.removeEventListener("keydown", esckey);
  document.body.addEventListener("keydown", shortkey);
  // console.log("short activated");
}
