use regex::Regex;

pub fn escape_page_title(title: &String) -> String {
    let re = Regex::new(r"[\s\u{200B}]").unwrap();
    re.replace_all(title, "_").to_string()
}
