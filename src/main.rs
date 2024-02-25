mod model;
mod view;

use glob::glob;
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::event::EventResult;
use cursive::views::*;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let mut select = SelectView::new()
        .h_align(HAlign::Center);
    let content = glob("morgues/*.txt")
        .expect("Failed to read morgues directory")
        .map(|path| match path {
            Ok(path) => path.display().to_string(),
            Err(e) => panic!()
        });
    select.add_all_str(content);

    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    siv.add_layer(select.scrollable());
    siv.run();
}
