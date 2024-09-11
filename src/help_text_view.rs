use cursive::{
    theme::{BaseColor, Color, Style},
    utils::{markup::StyledString, span::SpannedString},
    views::TextView,
};

fn build_help_text_with_style(
    spanned_strings: &mut SpannedString<Style>,
    action: &str,
    description: &str,
) {
    spanned_strings.append_styled(action, Color::Dark(BaseColor::Blue));
    spanned_strings.append_styled(description, Color::Dark(BaseColor::Black));
}

pub fn build_help_text_view() -> TextView {
    let mut styled = StyledString::new();

    build_help_text_with_style(&mut styled, "<X>", " Hide selected clipboard history item ");
    build_help_text_with_style(&mut styled, "<Alt+X>", " Remove selected item from history ");
    build_help_text_with_style(&mut styled, "<Ctrl+X>", " Clear clipboard history ");

    build_help_text_with_style(
        &mut styled,
        "<C>",
        " Copy the selected item to the clipboard ",
    );

    TextView::new(styled)
}
