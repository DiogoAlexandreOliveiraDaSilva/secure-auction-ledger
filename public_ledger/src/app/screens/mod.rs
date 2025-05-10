pub(crate) mod auction_screen;
pub(crate) mod bid_screen;
pub(crate) mod block_screen;
pub(crate) mod create_screen;
pub(crate) mod initial_screen;
pub(crate) mod join_screen;
pub(crate) mod menu_screen;
pub(crate) mod selection_screen;

pub enum AppState {
    Initial,
    Selection,
    Join,
    Menu,
    Auction,
    Create,
    Block,
    Bid,
}
