const CONSONANTS: &str = "bcdfgjklmnpqstvxzhrwy";
const USSY: &str = "ussy";

fn is_consonant(s: &str, index: usize) -> bool {
    let c = s.chars().nth(index).unwrap();
    CONSONANTS.contains(c)
}
enum Stage {
    Consonants,
    Vowels,
    Done,
}

///Adds "ussy" to the end of a string
//
/// # Examples
///
/// ```
/// use russy::mussy::fussy;
/// let rust = "rust".to_string();
///
/// assert_eq!("russy", fussy(&rust));
/// ```
pub fn fussy(s: &str) -> String {
    if s.is_ascii() && s.chars().all(|x| x.is_alphanumeric() || x == ' ') {
        let mut res = s.to_owned();
        let mut stage = Stage::Consonants;

        let mut i = res.len();
        loop {
            i -= 1;
            match stage {
                Stage::Consonants => {
                    if is_consonant(&res, i) {
                        res.remove(i);
                    } else {
                        stage = Stage::Vowels;
                        i += 1;
                    }
                }
                Stage::Vowels => {
                    if !is_consonant(&res, i) {
                        res.remove(i);
                    } else {
                        stage = Stage::Done;
                        i += 1;
                    }
                }
                Stage::Done => {
                    res.push_str(USSY);
                    break;
                }
            };
            if i == 0 {
                break;
            }
        }
        return res;
    }
    return "".to_string();
}
