mod data;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Zi {
    pub strokes: i64,
    pub pinyin_ton: &'static str,
    pub unicode: &'static str,
    pub hanzi: char,
    pub sens: &'static str,
}

pub fn getsize() -> usize {
    return data::ZIDIAN.len();
}

pub fn pylist(pinyin: &str) -> Vec<Zi> {
    let last_char = pinyin.chars().last().unwrap();
    let cond = matches!(last_char, '0'..='4');

    let mut vec = Vec::<Zi>::new();
    let n = data::ZIDIAN.len();
    let mut i = 0;

    while i < n {
        let dbzi = &data::ZIDIAN[i];
        let dbpt = dbzi.pinyin_ton;
        let compare = if cond { dbpt } else { &dbpt[0..dbpt.len() - 1] };
        if compare == pinyin {
            let unic = dbzi.unicode;
            let unicodestr = u32::from_str_radix(unic, 16).unwrap();

            let zi = Zi {
                pinyin_ton: dbzi.pinyin_ton,
                unicode: unic,
                hanzi: char::from_u32(unicodestr).unwrap(),
                sens: dbzi.sens,
                strokes: dbzi.strokes,
            };
            vec.push(zi);
        }
        i += 1;
    }
    vec.sort();
    vec
}

pub fn zilist(carac: &str) -> Vec<Zi> {
    let zi = carac.chars().next().unwrap();
    let mut vec = Vec::<Zi>::new();
    let n = data::ZIDIAN.len();
    let mut i = 0;
    while i < n {
        let dbzi = &data::ZIDIAN[i];
        let zicode: u32 = u32::from_str_radix(dbzi.unicode, 16).unwrap();
        let hanzi = char::from_u32(zicode).unwrap();
        if hanzi == zi {
            let zi = Zi {
                pinyin_ton: dbzi.pinyin_ton,
                unicode: dbzi.unicode,
                hanzi: zi,
                sens: dbzi.sens,
                strokes: dbzi.strokes,
            };
            vec.push(zi);
        }
        i += 1;
    }
    vec.sort();
    vec
}

pub fn strokelist(nbstroke: i64) -> Vec<Zi> {
    let mut vec = Vec::<Zi>::new();
    let n = data::ZIDIAN.len();
    let mut i = 0;
    while i < n {
        let dbzi = &data::ZIDIAN[i];
        let zicode: u32 = u32::from_str_radix(dbzi.unicode, 16).unwrap();
        let hanzi = char::from_u32(zicode).unwrap();

        if dbzi.strokes == nbstroke {
            let zi = Zi {
                pinyin_ton: dbzi.pinyin_ton,
                unicode: dbzi.unicode,
                hanzi: hanzi,
                sens: dbzi.sens,
                strokes: nbstroke,
            };
            vec.push(zi);
        }
        i += 1;
    }
    vec.sort();
    vec
}

pub fn list() -> Vec<Zi> {
    let mut vec = Vec::<Zi>::new();
    let n = data::ZIDIAN.len();
    let mut i = 0;
    while i < n {
        let dbzi = &data::ZIDIAN[i];
        let unic = dbzi.unicode;
        let unicodestr = u32::from_str_radix(unic, 16).unwrap();

        let zi = Zi {
            pinyin_ton: dbzi.pinyin_ton,
            unicode: unic,
            hanzi: char::from_u32(unicodestr).unwrap(),
            sens: dbzi.sens,
            strokes: dbzi.strokes,
        };
        vec.push(zi);
        i += 1
    }
    vec.sort();
    vec
}
