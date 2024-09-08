use cursive::align::HAlign;
use cursive::view::Scrollable;
use cursive::views::{ScrollView, SelectView};
use cursive::Cursive;

use crate::clipboard_content::ClipboardContent;

pub fn build_select_content_view<F>(
    clipboard_content: Vec<ClipboardContent>,
    on_submit_cb: F,
) -> ScrollView<SelectView>
where
    F: 'static + Fn(&mut Cursive, &String) + Send + Sync,
{
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center);

    for cc in clipboard_content {
        select.add_item(cc.id, cc.content);
    }

    select.set_on_select(on_submit_cb);    
    select.scrollable()
}
