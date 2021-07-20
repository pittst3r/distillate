pub fn replace_last_bsp_with_nbsp(string: &str) -> String {
    let string = String::from(string);
    let bsp = " ";
    let nbsp = "\u{00A0}";
    let replace = move |mut text: String, i| {
        text.replace_range(i..(i + bsp.len()), nbsp);
        text
    };

    match string.rfind(bsp) {
        Some(i) => replace(string, i),
        None => string,
    }
}
