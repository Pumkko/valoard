use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::views::{OnEventView, SelectView};
use cursive::Cursive;

pub fn build_select_content_view<F>(on_submit_cb: F) -> OnEventView<SelectView>
where
    F: 'static + Fn(&mut Cursive, &String) + Send + Sync,
{
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    let content = include_str!("assets/cities.txt");
    select.add_all_str(content.lines());

    // Sets the callback for when "Enter" is pressed.
    //select.set_on_submit(on_submit_cb);

    select.set_on_select(on_submit_cb);

    // Let's override the `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    select
}
