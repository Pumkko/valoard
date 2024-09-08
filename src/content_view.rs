use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    view::{scroll::Scroller, Scrollable},
    views::{Dialog, OnEventView, Panel, TextView},
    CursiveRunnable, With,
};

pub trait ContentList {
    fn add_content_list(&mut self);
}

impl ContentList for CursiveRunnable {
    fn add_content_list(&mut self) {
        // Read some long text from a file.
        let content = include_str!("assets/lorem.txt");

        // The text is too long to fit on a line, so the view will wrap lines,
        // and will adapt to the terminal size.
        self.add_fullscreen_layer(
            Dialog::around(Panel::new(
                TextView::new(content)
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
                    }),
            ))
            .title("Unicode and wide-character support")
            // This is the alignment for the button
            .h_align(HAlign::Center)
            .button("Quit", |s| s.quit()),
        );
    }
}
