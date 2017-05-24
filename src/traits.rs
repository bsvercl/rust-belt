use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings, Events};
use piston::input::Input;
use std::sync::Mutex;

lazy_static! {
    static ref EVENTS: Mutex<Events> = Mutex::new(Events::new(EventSettings::default()));
}

pub trait GlutinWindowExt {
    fn next(&mut self) -> Option<Input>;
}

impl GlutinWindowExt for GlutinWindow {
    fn next(&mut self) -> Option<Input> {
        EVENTS.lock().unwrap().next(self)
    }
}
