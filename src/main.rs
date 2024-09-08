#![warn(clippy::pedantic, clippy::all)]
#![allow(clippy::module_name_repetitions)]

mod clipboard_content;
mod content_view;
mod select_content_view;

use clipboard_content::get_clipboard_content;
use content_view::build_content_view;
use cursive::align::HAlign;
use cursive::views::{Dialog, DummyView, LinearLayout, TextView};
use cursive::{traits::Resizable, Cursive};
use select_content_view::build_select_content_view;

fn main() {
    let mut siv = cursive::default();

    let clipboard_content = get_clipboard_content();

    // TODO: add specific behavior when the clipboard is empty
    let content_view = build_content_view(clipboard_content[0].content.clone());
    let select_content_view = build_select_content_view(clipboard_content, show_next_window);

    siv.add_global_callback('q', Cursive::quit);

    // We'll create a dialog with a TextView serving as a title
    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select_content_view.full_height().min_width(30))
                .child(DummyView.fixed_width(3))
                .child(content_view.full_width()),
        )
        .title("Valoard")
        .h_align(HAlign::Center)
        .full_screen(),
    );

    siv.run();
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, content: &String) {
    siv.call_on_name("clipboard_content", |view: &mut TextView| {
        view.set_content(content);
    });
}
