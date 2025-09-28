pub fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

pub fn index_html() -> String {
    templated("Hallo Papa, Ich bin in einem ESP-32 Microcontroller gefangen. Hilfe!")
}
