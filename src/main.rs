use sixtyfps::{Model, ModelHandle, VecModel, SharedString};
use std::rc::Rc;

sixtyfps::include_modules!();

fn main() {
    let ui = AppWindow::new();
    let mut tiles: Vec<DayData> = ui.get_days().iter().collect();

    // Generate days of the month
    // Duplicate them to ensure that we have pairs
    tiles.pop();
    tiles.pop();
    for i in 1..31 {
        tiles.push(DayData {
            daynum: SharedString::from(i.to_string()),
        });
    }
    //tiles.extend(tiles.clone());

    let tiles_model = Rc::new(VecModel::from(tiles));
    ui.set_days(ModelHandle::new(tiles_model.clone()));
    let _appwin_weak = ui.as_weak();
    ui.run();
}
