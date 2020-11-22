use super::ClipboardCtx;
// use super::Targets;

// use xcb::base::Event;
// use xcb::ffi::base::xcb_generic_event_t;

pub struct Setter<'a> {
    ctx: &'a ClipboardCtx,
}

impl<'a> Setter<'a> {
    pub fn new(ctx: &'a ClipboardCtx) -> Self {
        Setter { ctx }
    }
}
