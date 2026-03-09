use crate::dbase;
use web_sys::window;

pub fn ziprinter(query: &str, vec: Vec<dbase::Zi>) {
    let cont = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("content")
        .unwrap();
    if vec.len() == 0 {
        cont.set_inner_html(&format!("No result for query \"{}\"", query));
    } else {
        let mut print: String = format!("Result for query \"{}\" :<br />", query);
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
pub fn printcandidatelist(chain: &str) {
    let answer = dbase::getcandidatelist(chain);
    let mut resp: String;
    if answer.is_empty() {
        resp = "<br /><br />No hanzi available for request".to_owned()
    } else {
        resp = String::from("<br />Select one hanzi from this list:<br>");
        resp.push_str(&answer)
    }
    let cont = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("zilist")
        .unwrap();
    cont.set_inner_html(&resp);
}
pub fn parseprinter(inputstring: &str) {
    let chain = ammonia::clean(inputstring);
    let mut resp: String;
    resp = format!(
        "<p>Input string:</p><p class='hanzi'>{}</p><p>Parsed string:</p>",
        chain
    );
    let mut chars = chain.chars();
    let mut parsed = String::new();
    let mut unknown = Vec::<String>::new();
    let mut nonzi: bool = false;
    while let Some(carac) = chars.next() {
        // 1. If carac is not a chinese character or is a punctuation mark, simply append it to parsed
        if (carac as i64) < 0x2000
            || "。，“”（）、《》—；：！？「」 【】『』％‘’•".find(carac) != None
        {
            if nonzi {
                parsed = format!("{}{}", parsed, carac)
            } else {
                parsed = format!("{}   {}", parsed, carac); // insert spaces before first non zi character
                nonzi = true;
            }
        } else {
            nonzi = false; // this is a zi: reset nonzi
            // 2. get all pinyin for the carac character in the database
            let disp = dbase::zilist(&carac.to_string());
            if disp.len() > 0 {
                // 3. The character exists in the database: give all pinyin separated by /
                // parsed = format!("{}{}", parsed, " "); // simpler as follows:
                parsed = format!("{} ", parsed); // insert space for better readability
                for (i, py) in disp.iter().enumerate() {
                    if i > 0 {
                        parsed = format!("{}{}", parsed, "/");
                    }
                    parsed = format!("{}{}", parsed, py.pinyin_ton);
                }
            } else {
                // 4. The character is not in the base: add it to the unknown Vec
                // 5. and append it as such (unparsed) to parsed
                unknown.push(carac.to_string());
                parsed = format!("{} {}", parsed, carac);
            }
        }
    }
    resp.push_str(&format!("<p>{}</p>", parsed));
    if unknown.len() == 0 {
        resp.push_str("<p>No unknown zi in input string</p>")
    } else {
        resp.push_str("<p>The following zi are not in the database:</p>");
        for zi in unknown {
            resp.push_str(&format!("<li class='hanzi'>{}</li>", zi));
        }
    }
    let cont = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("content")
        .unwrap();
    cont.set_inner_html(&resp);
}
