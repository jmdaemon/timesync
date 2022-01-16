//use sixtyfps::{Model};
//use sixtyfps::{Model, ModelHandle, Timer, VecModel};
use sixtyfps::{Model, ModelHandle, VecModel};
use std::rc::Rc;
//use std::time::Duration;

//sixtyfps::include_modules!();
sixtyfps::include_modules!();

//sixtyfps::sixtyfps! {
    //import { Days, AppWindow } from "src/ui/calendar.60";
//}

fn main() {
    let ui = AppWindow::new();

    //let mut tiles: Vec<DayData> = ui.get_days().iter().collect();

    let mut tiles: Vec<DayData> = ui.get_days().iter().collect();
    //let mut tiles: Vec<DayData> = ui.get_days().into_iter().collect();
    // Duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    let tiles_model = Rc::new(VecModel::from(tiles));
    ui.set_days(ModelHandle::new(tiles_model.clone()));
    let _appwin_weak = ui.as_weak();
    ui.run();
}
