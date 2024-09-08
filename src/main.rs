mod select_content_view;

use cursive::align::HAlign;
use cursive::theme::{BorderStyle, ColorStyle, StyleType};
use cursive::view::Margins;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::{traits::*, Cursive};
use select_content_view::build_select_content_view;

// This example uses a LinearLayout to stick multiple views next to each other.

fn main() {
    let mut siv = cursive::default();

    // Some description text. We want it to be long, but not _too_ long.
    let text = "This is a very simple example of linear layout. Two views \
                are present, a short title above, and this text. The text \
                has a fixed width, and the title is centered horizontally.";

    // Let's add a ResizedView to keep the list at a reasonable size
    // (it can scroll anyway).
    /*self.add_layer(
        Dialog::around(select_content_view.scrollable().fixed_size((20, 10))).title("Where are you from?"),
    );*/
    let select_content_view = build_select_content_view(show_next_window);

    // We'll create a dialog with a TextView serving as a title
    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select_content_view.scrollable().full_height().min_width(30))
                .child(DummyView.fixed_height(1))
                .child(TextView::new(text).with_name("clipboard_content"))
                .full_width(),
        )
        .button("Quit", |s| s.quit())
        .h_align(HAlign::Center)
        .full_screen(),
    );

    siv.run();
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, city: &String) {
    siv.call_on_name("clipboard_content", |view: &mut TextView| {
        view.set_content(city);
    });
}
