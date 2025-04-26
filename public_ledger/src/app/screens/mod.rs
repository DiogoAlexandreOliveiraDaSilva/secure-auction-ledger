
pub(crate) mod initial_screen;
pub(crate) mod selection_screen;
pub(crate) mod join_screen;
pub(crate) mod menu_screen;


pub enum AppState {
    Initial,
    Selection,
    Join,
    Menu,   
}
