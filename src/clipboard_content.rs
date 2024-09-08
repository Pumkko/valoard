pub struct ClipboardContent {
    pub id: String,
    pub content: String,
}

pub fn get_clipboard_content() -> Vec<ClipboardContent> {
    // Read some long text from a file.
    let large_content = include_str!("assets/lorem.txt");

    vec![
        ClipboardContent {
            content: String::from("hello"),
            id: String::from("2024-09-08 09h41:25 -04h00"),
        },
        ClipboardContent {
            content: String::from("world"),
            id: String::from("2024-09-08 09h45:25 -04h00"),
        },
        ClipboardContent {
            content: large_content.to_string(),
            id: String::from("2024-09-08 09h48:55 -04h00"),
        }
    ]
}
