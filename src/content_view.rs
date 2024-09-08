use cursive::{
    event::{EventResult, Key},
    view::{scroll::Scroller, Nameable, Scrollable},
    views::{NamedView, OnEventView, ScrollView, TextView},
    With,
};

pub fn build_content_view(initial_value: String) -> OnEventView<ScrollView<NamedView<TextView>>> {
    // The text is too long to fit on a line, so the view will wrap lines,
    // and will adapt to the terminal size.

    TextView::new(initial_value)
        .with_name("clipboard_content")
        .scrollable()
        .wrap_with(OnEventView::new)
        .on_pre_event_inner(Key::PageUp, |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_up() {
                scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
            }
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner(Key::PageDown, |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_down() {
                scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
            }
            Some(EventResult::Consumed(None))
        })
}
