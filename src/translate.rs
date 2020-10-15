

pub fn translate(str: String) -> String {
    translate_month(str)
} 

fn translate_month(str: String) -> String {
    let month_replacements: Vec<(&str, &str)> = vec![
    ("January", "Januar"),
    ("February", "Februar"),
    ("March", "März"),

    ("May", "Mai"),
    ("June", "Juni"),
    ("July", "Juli"),

    ("October", "Oktober"),


    ("December", "Dezember")];

    replace_all(str, month_replacements)

}

fn replace_all(str: String, replaces: Vec<(&str, &str)>) -> String {
    let mut result = str;
    for (pattern, value) in replaces.into_iter() {
        result = result.replace(pattern, value);
    }
    return result;
}
