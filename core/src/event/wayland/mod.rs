mod layer;
mod output;
mod popup;
mod seat;
mod session_lock;
mod window;

use crate::{time::Instant, window::Id};
use sctk::reexports::client::protocol::{
    wl_output::WlOutput, wl_seat::WlSeat, wl_surface::WlSurface,
};

pub use layer::*;
pub use output::*;
pub use popup::*;
pub use seat::*;
pub use session_lock::*;
pub use window::*;

/// wayland events
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// layer surface event
    Layer(LayerEvent, WlSurface, Id),
    /// popup event
    Popup(PopupEvent, WlSurface, Id),
    /// output event
    Output(OutputEvent, WlOutput),
    /// window event
    Window(WindowEvent),
    /// Seat Event
    Seat(SeatEvent, WlSeat),
    /// Session lock events
    SessionLock(SessionLockEvent),
    /// Frame events
    Frame(Instant, WlSurface, Id),
    /// Request Resize
    RequestResize,
}
