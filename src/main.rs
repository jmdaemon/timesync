use sixtyfps::{Model, ModelHandle, VecModel, SharedString};
use std::rc::Rc;

sixtyfps::include_modules!();

fn main() {
    let ui = AppWindow::new();

    let mut days: Vec<DayData> = ui.get_days().iter().collect();

    // Generate days of the month
    for i in 1..31 {
        days.push(DayData {
            daynum: SharedString::from(i.to_string()),
        });
    }

    let days_model = Rc::new(VecModel::from(days));
    ui.set_days(ModelHandle::new(days_model.clone()));
    let _appwin_weak = ui.as_weak();
    ui.run();
}
