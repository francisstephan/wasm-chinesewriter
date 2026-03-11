// src/lib.rs
mod dbase;
mod writers;

use std::cell::Cell;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlFormElement, HtmlInputElement, console, window};

struct Storeclosure {
    closure: Closure<dyn FnMut(Event)>,
}
// we use this struct because Cell (line 15 herebelow) requires a Sized element
// meaning we cannot call Cell(Closure) but we can call Cell(Storeclosure)
// see https://dev-doc.rust-lang.org/stable/std/cell/struct.Cell.html
thread_local! {  // https://www.sitepoint.com/rust-global-variables/
    static CLOSURE: Cell<Storeclosure> = Cell::new(Storeclosure {
        closure: Closure::<dyn FnMut(Event)>::once(move |event: Event| {
            event.prevent_default(); }) // dummy closure to keep the compiler happy
    });
}

#[wasm_bindgen]
extern "C" {
    fn minusesc_plusshort(); // defined in chinesewriter.js
    // remove esc key event listener
    // add p,z and s shortcut keys event listener
    // (see https://github.com/francisstephan/wasm-chinesewriter/blob/main/README.md )
}

#[wasm_bindgen]
pub fn getsize() -> String {
    dbase::getsize()
}

#[wasm_bindgen]
pub fn listdic() -> () {
    writers::ziprinter("dictionary list", dbase::list())
}

#[wasm_bindgen]
pub fn getpyform() -> String {
    let form = r##"
        <form id="pyinput" autocomplete="off">
		    <label for="pinyin">Pinyin+tone (using pattern ^[a-z,ü]+[0-4]?) :</label>
		    <input id="pinyin" name="pinyin_ton" type="text" pattern="^[a-z,ü]+[0-4]?" required>
		    <button class="menubouton" type="submit">Submit</button>
	  </form>
	  <button id="cancel" class="menubouton">Cancel</button>
	"##;
    String::from(form)
} // autofocus does not work, will generate warning in console, see index.js l.53
// 'required' makes sure that the input field is tested for validity before submit
#[wasm_bindgen]
pub fn getziform() -> String {
    let form = r##"
	  <form id="ziinput" autocomplete="off">
		    <label for="carac">Character:</label>
		    <input id="carac" name="carac" type="text" minlength="1" maxlength="1" required>
		    <button class="menubouton" type="submit">Submit</button>
	  </form>
	  <button id="cancel" class="menubouton">Cancel</button>
	"##;
    String::from(form)
}

#[wasm_bindgen]
pub fn getstrokeform() -> String {
    let form = r##"
	  <form id="strokeinput" autocomplete="off">
		    <label for="stroke">Number of strokes:</label>
		    <input id="stroke" name="stroke" type="number"  min="1" max="30" required>
		    <button class="menubouton" type="submit">Submit </button>
	  </form>
	  <button id="cancel" class="menubouton">Cancel</button>
	"##;
    String::from(form)
}

#[wasm_bindgen]
pub fn whz() -> String {
    // hx-post="/candidatelist" hx-target="#zilist" hx-swap="innerHTML"
    let form = r##"
        <h2>
	    Use latin keyboard, write text in chinese characters (hanzi, 汉字)
        </h2>
        <p>1. Enter pinyin with tone in the "Enter" textarea below. A list of possible hanzi appears.<br />
           2. Select hanzi from list by clicking on it.<br />
           The selected zi gets added to the <b>Result hanzi text</b>, which you may copy to clipboard, send to Google translate, etc.<br />
           To add more hanzi to the text, repeat steps 1 & 2 again</p>
        <form id="postzi" autocomplete="off">
            <label for="pinyin">Enter pinyin+tone (press / or space after pinyin if tone unknown) :</label>
            <input type='text' id='pinyin' name='pinyin_ton' pattern="^[a-z,ü]+[0-4]?" size="10" oninput='convertToZi()'>
            <button id="subpy" style="display:none" type="submit"></button>
        </form>
        <p id="resultat"><b>Result hanzi text :</b><input type='text' id='zistring' size='60'></p>
        <button class = "Addzi" onclick='copyTextToClipboard()' >Copy to clipboard</button>
        <button class = "Addzi" onclick='reset()' >Reset</button>
        <button class = "Addzi" onclick="lookup(document.getElementById('zistring').value)" >Google Translate text</button>
        <button class = "Addzi" onclick="lookupWrittenChinese(document.getElementById('zistring').value)" >Lookup Hanzii dictionary</button>
        <div id="zilist"></div>
    "##;
    String::from(form)
}

#[wasm_bindgen]
pub fn getparseform() -> String {
    let form = r##"
	    <h2 id="formhead">Enter hanzi string to parse :</h2>
        <form id="getparse" autocomplete="off" >
		    <input id="zistr" name="zistr" type="text" required size="80" minlength="1" maxlength="400">
		    <button class="menubouton" type="submit">Click to submit </button>
	    </form>
		<button id="cancel" class="menubouton">Cancel</button>
	"##;
    String::from(form)
}

// see https://deepwiki.com/rustwasm/wasm-bindgen/2.3-closure-system
#[wasm_bindgen]
pub fn connectpyform() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let form = document
        .get_element_by_id("pyinput")
        .unwrap()
        .dyn_into::<HtmlFormElement>()?;
    let form_ref = form.clone(); // will be moved into the closure

    let closure = Closure::<dyn FnMut(Event)>::once(move |event: Event| {
        event.prevent_default(); // stop page reload
        let input = &form_ref
            .get_with_name("pinyin_ton")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if !input.check_validity() {
            // performs the Html checks included in the form (line 36 hereabove)
            input.report_validity();
            return;
        }
        let binding = input.value();
        let pinyin = binding.as_str();
        writers::ziprinter(pinyin, dbase::pylist(pinyin));
        minusesc_plusshort(); // defined in chinesewriter.js
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    // closure.forget(); // keep closure alive : avoid this if memory leaks are an issue
    let storeclosure = Storeclosure { closure: closure };
    CLOSURE.set(storeclosure); // keep closure alive (otherwise it is dropped when leaving scope)
    // The last defined closure remains stored in CLOSURE, until it gets replaced by a new one
    // This works because there is at most one form present at any given time in this program
    Ok(())
}

#[wasm_bindgen]
pub fn connectziform() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let form = document
        .get_element_by_id("ziinput")
        .unwrap()
        .dyn_into::<HtmlFormElement>()?;
    let form_ref = form.clone();
    let closure = Closure::<dyn FnMut(Event)>::once(move |event: Event| {
        event.prevent_default(); // stop page reload

        console::log_1(&"in ziform callback closure".into());
        let input = &form_ref
            .get_with_name("carac")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if !input.check_validity() {
            // check that there is exactly one char (line 49 hereabove)
            input.report_validity();
            return;
        }

        let binding = input.value();
        let carac = binding.as_str();
        writers::ziprinter(carac, dbase::zilist(carac));
        minusesc_plusshort(); // defined in chinesewriter.js
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    let storeclosure = Storeclosure { closure: closure };
    CLOSURE.set(storeclosure);

    Ok(())
}

#[wasm_bindgen]
pub fn connectstrokeform() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let form = document
        .get_element_by_id("strokeinput")
        .unwrap()
        .dyn_into::<HtmlFormElement>()?;
    let form_ref = form.clone();
    let closure = Closure::<dyn FnMut(Event)>::once(move |event: Event| {
        event.prevent_default(); // stop page reload

        let input = &form_ref
            .get_with_name("stroke")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if !input.check_validity() {
            // check that input is a number between 1 and 30 (line 62 hereabove)
            console::log_1(&"in strokeform validity control".into()); // should never print
            input.report_validity();
            return;
        }

        let binding = input.value();
        let carac = binding.as_str();
        console::log_1(&format!("Stroke number :{}", input.value()).into());
        let nbstroke: i64 = carac.parse().unwrap(); // we checked number in form validation
        let mess = format!("Characters with {} strokes", nbstroke);
        writers::ziprinter(&mess, dbase::strokelist(nbstroke));
        minusesc_plusshort(); // defined in chinesewriter.js
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    let storeclosure = Storeclosure { closure: closure };
    CLOSURE.set(storeclosure);

    Ok(())
}

#[wasm_bindgen]
pub fn connectwhzform() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let form = document
        .get_element_by_id("postzi")
        .unwrap()
        .dyn_into::<HtmlFormElement>()?;
    let form_ref = form.clone(); // will be moved in the closure

    let closure = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        event.prevent_default(); // stop page reload
        let input = &form_ref
            .get_with_name("pinyin_ton")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if !input.check_validity() {
            // performs the Html checks included in the form (line 83 hereabove)
            input.report_validity();
            return;
        }
        let binding = input.value();
        let pinyin = binding.as_str();
        console::log_1(&format!("Pinyin :{}", input.value()).into());
        writers::printcandidatelist(pinyin);
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    // closure.forget(); // keep closure alive : avoid this if memory leaks are an issue
    let storeclosure = Storeclosure { closure: closure };
    CLOSURE.set(storeclosure); // keep closure alive (otherwise it is dropped when leaving scope)
    // The last defined closure remains stored in CLOSURE, until it gets replaced by a new one
    // This works because there is at most one form present at any given
    Ok(())
}

#[wasm_bindgen]
pub fn connectparseform() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let form = document
        .get_element_by_id("getparse")
        .unwrap()
        .dyn_into::<HtmlFormElement>()?;
    let form_ref = form.clone(); // will be moved into the closure

    let closure = Closure::<dyn FnMut(Event)>::once(move |event: Event| {
        event.prevent_default(); // stop page reload
        let input = &form_ref
            .get_with_name("zistr")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if !input.check_validity() {
            // performs the Html checks included in the form (line 36 hereabove)
            input.report_validity();
            return;
        }
        let binding = input.value();
        let chain = binding.as_str();
        writers::parseprinter(chain);
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    // closure.forget(); // keep closure alive : avoid this if memory leaks are an issue
    let storeclosure = Storeclosure { closure: closure };
    CLOSURE.set(storeclosure); // keep closure alive (otherwise it is dropped when leaving scope)
    // The last defined closure remains stored in CLOSURE, until it gets replaced by a new one
    // This works because there is at most one form present at any given time in this program
    Ok(())
}
