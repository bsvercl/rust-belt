use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings, Events};
use piston::input::Input;

pub trait GlutinWindowExt {
    fn next(&mut self) -> Option<Input>;
}

impl GlutinWindowExt for GlutinWindow {
    fn next(&mut self) -> Option<Input> {
        use std::sync::Mutex;
        lazy_static! {
            static ref EVENTS: Mutex<Events> = Mutex::new(Events::new(EventSettings::default()));
        }
        EVENTS.lock().unwrap().next(self)
    }
}
