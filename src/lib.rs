// src/lib.rs
mod dbase;

use std::cell::Cell;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlFormElement, HtmlInputElement, console, window};

struct Storeclosure {
    closure: Closure<dyn FnMut(Event)>,
}
// we use a struct because Cell requires a Sized element
// see https://dev-doc.rust-lang.org/stable/std/cell/struct.Cell.html

thread_local! {
    static CLOSURE: Cell<Storeclosure> = Cell::new(Storeclosure {
        closure: Closure::<dyn FnMut(Event)>::once(move |event: Event| {
            event.prevent_default(); }) // dummy closure to keep the compiler happy
    });
}

#[wasm_bindgen]
pub fn getsize() -> usize {
    dbase::getsize()
}

#[wasm_bindgen]
pub fn listdic() -> () {
    ziprinter("dictionary list", dbase::list())
}

#[wasm_bindgen]
pub fn cancel() {
    let document = window().unwrap().document().unwrap();
    let cont = document.get_element_by_id("content").unwrap();
    cont.set_inner_html("Form canceled.");
}

#[wasm_bindgen]
pub fn getpyform() -> String {
    let form = r##"
        <form id="pyinput">
		    <label for="pinyin">Pinyin+tone (using pattern ^[a-z,ü]+[0-4]?) :</label>
		    <input id="pinyin" name="pinyin_ton" type="text" pattern="^[a-z,ü]+[0-4]?" autofocus>
		    <button class="menubouton" type="submit">Submit</button>
			<button id="cancel" class="menubouton">Cancel</button>
	  </form>
	"##;
    String::from(form)
}

#[wasm_bindgen]
pub fn getziform() -> String {
    let form = r##"
	  <form id="ziinput">
		    <label for="carac">Character:</label>
		    <input id="carac" name="carac" type="text" autofocus required minlength="1" maxlength="1">
		    <button class="menubouton" type="submit">Submit</button>
			<button id="cancel" class="menubouton">Cancel</button>
	  </form>
	"##;
    String::from(form)
}

#[wasm_bindgen]
pub fn getstrokeform() -> String {
    let form = r##"
	  <form id="strokeinput">
		    <label for="stroke">Number of strokes:</label>
		    <input id="stroke" name="stroke" type="number"  min="1" max="30">
		    <button class="menubouton" type="submit">Submit </button>
			<button id="cancel" class="menubouton">Cancel</button>
	  </form>
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
    let form_ref = form.clone(); // will be moved in the Closure

    let closure = Closure::<dyn FnMut(Event)>::once(move |event: Event| {
        event.prevent_default(); // stop page reload
        let input = &form_ref
            .get_with_name("pinyin_ton")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if !input.check_validity() {
            // performs the Html checks included in the form (line 43 hereabove)
            input.report_validity();
            return;
        }
        let binding = input.value();
        let pinyin = binding.as_str();
        ziprinter(pinyin, dbase::pylist(pinyin));
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    // closure.forget(); // keep closure alive : avoid this if memory leaks are an issue
    let storeclosure = Storeclosure { closure: closure };
    CLOSURE.set(storeclosure);

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
            // check that there is exactly one char (line 56 hereabove)
            input.report_validity();
            return;
        }

        let binding = input.value();
        let carac = binding.as_str();
        ziprinter(carac, dbase::zilist(carac));
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    // closure.forget(); // keep closure alive
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
            // check that input is a number between 1 and 30 (line 69 hereabove)
            input.report_validity();
            return;
        }

        let binding = input.value();
        let carac = binding.as_str();
        console::log_1(&format!("Stroke number :{}", input.value()).into());
        let nbstroke: i64 = carac.parse().unwrap(); // we already checked numeric validity
        let mess = format!("Characters with {} strokes", nbstroke);
        ziprinter(&mess, dbase::strokelist(nbstroke));
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    // closure.forget(); // keep closure alive
    let storeclosure = Storeclosure { closure: closure };
    CLOSURE.set(storeclosure);

    Ok(())
}

fn ziprinter(query: &str, vec: Vec<dbase::Zi>) {
    let document = window().unwrap().document().unwrap();
    let mut print: String;
    let cont = document.get_element_by_id("content").unwrap();

    if vec.len() == 0 {
        cont.set_inner_html(&format!("No result for query \"{}\"", query));
    } else {
        print = format!("Result for query \"{}\" :<br />", query);
        print.push_str("<table><tr><td>Strokes</td><td>Pinyin</td><td>Unicode</td><td>Character</td><td>Translation</td></tr>");
        for zi in vec {
            print.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td>",
                zi.strokes, zi.pinyin_ton, zi.unicode, zi.hanzi, zi.sens
            ));
        }
        cont.set_inner_html(&print);
    }
}
