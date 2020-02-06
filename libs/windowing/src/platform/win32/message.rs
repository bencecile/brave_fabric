use winapi::{
    shared::minwindef::{LPARAM, UINT, WPARAM},
    um::winuser,
};
use crate::{Event};

pub fn convert_message(message: UINT, _w_param: WPARAM, _l_param: LPARAM) -> Option<Event> {
    match message {
        winuser::WM_CLOSE => Some(Event::WindowClosed),
        _ => None,
    }
}
