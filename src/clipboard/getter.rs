use super::ClipboardCtx;
use super::Targets;
use crate::config::Config;
use xcb::base::Event;
use xcb::ffi::base::xcb_generic_event_t;

const LEN_PROPERTY_GET: u32 = std::u32::MAX;

pub struct Getter<'a> {
    ctx: &'a ClipboardCtx,
    targets: Targets,
    xfixes_event_base: u8,
}

pub enum GetterError {
    UnknownTarget,
}

pub enum ProcessState {
    Done,
    WrongTarget,
    SkipEvent,
    GettingLongValue,
    ClipboardChanged,
}

impl<'a> Getter<'a> {
    pub fn new(config: &Config, ctx: &'a ClipboardCtx) -> Self {
        let targets = Targets::new(&ctx.connection, config);

        let xfixes = xcb::query_extension(&ctx.connection, "XFIXES")
            .get_reply()
            .unwrap();
        assert!(xfixes.present());
        xcb::xfixes::query_version(&ctx.connection, 5, 0);
        let xfixes_event_base = xfixes.first_event();

        Getter {
            ctx,
            targets,
            xfixes_event_base,
        }
    }

    fn send_get_req(&self) {
        xcb::convert_selection(
            &self.ctx.connection,
            self.ctx.window,
            self.ctx.selection_type,
            self.targets.get_current().atom,
            self.ctx.property,
            xcb::CURRENT_TIME,
        );
        self.ctx.connection.flush();
    }

    fn process_event(&self, event: Event<xcb_generic_event_t>, buf: &mut Vec<u8>) -> ProcessState {
        let etype = event.response_type();

        if etype == (self.xfixes_event_base + xcb::xfixes::SELECTION_NOTIFY) {
            // in other sources we use event.timestamp() as last arg for convert_selection,
            // but else it also work. Ok
            ProcessState::ClipboardChanged
        } else {
            match etype & !0x80 {
                xcb::SELECTION_NOTIFY => {
                    let eve = unsafe { xcb::cast_event::<xcb::SelectionNotifyEvent>(&event) };
                    let ev_prop = eve.property();

                    if ev_prop == xcb::ATOM_NONE {
                        ProcessState::WrongTarget
                    } else {
                        self.process_get_value(buf)
                    }
                }
                xcb::PROPERTY_NOTIFY => {
                    // should we implement it? It's just work
                    // eprintln!("[rclip] Not yet implemented");
                    ProcessState::SkipEvent
                }
                _ => {
                    // eprintln!("[rclip] Unknown etype: {}", etype);
                    ProcessState::SkipEvent
                }
            }
        }
    }
    fn process_get_value(&self, buf: &mut Vec<u8>) -> ProcessState {
        let reply = xcb::get_property(
            &self.ctx.connection,
            false,
            self.ctx.window,
            // in other sources we use event.property(),
            // but else it also work. Ok
            // ev_prop,
            self.ctx.property,
            xcb::ATOM_ANY,
            (buf.len() / 4) as u32,
            LEN_PROPERTY_GET,
        )
        .get_reply()
        .unwrap();

        if reply.type_() != self.targets.get_current().atom {
            ProcessState::WrongTarget
        } else {
            let val = reply.value();

            buf.extend_from_slice(val);
            if ((val.len() / 4) as u32) < LEN_PROPERTY_GET {
                ProcessState::Done
            } else {
                ProcessState::GettingLongValue
            }
        }
    }

    fn prepare_for_get(&mut self) {
        self.targets.restore();

        let screen_ptr = &self
            .ctx
            .connection
            .get_setup()
            .roots()
            .nth(self.ctx.screen as usize)
            .ok_or(xcb::base::ConnError::ClosedInvalidScreen)
            .unwrap();
        xcb::xfixes::select_selection_input(
            &self.ctx.connection,
            screen_ptr.root(),
            self.ctx.selection_type,
            xcb::xfixes::SELECTION_EVENT_MASK_SET_SELECTION_OWNER
                | xcb::xfixes::SELECTION_EVENT_MASK_SELECTION_CLIENT_CLOSE
                | xcb::xfixes::SELECTION_EVENT_MASK_SELECTION_WINDOW_DESTROY,
        );
        self.ctx.connection.flush();
    }

    /// Will wait until clibpoard changed. 'buf' parameter is where result buffer will be written.
    pub fn get_wait(&mut self, buf: &mut Vec<u8>) -> Result<String, GetterError> {
        self.prepare_for_get();

        loop {
            match self.ctx.connection.wait_for_event() {
                Some(event) => match self.process_event(event, buf) {
                    ProcessState::Done => {
                        // don't know why, but with some applications (flameshot, for example)
                        // clipboard does not changing without deleting property
                        xcb::delete_property(
                            &self.ctx.connection,
                            self.ctx.window,
                            self.ctx.property,
                        );
                        self.ctx.connection.flush();

                        break;
                    }
                    ProcessState::WrongTarget => match self.targets.roll_next() {
                        Ok(()) => self.send_get_req(),
                        Err(super::targets::RollError::BoundReached) => {
                            // empty clipboard. Probably just application that handled last
                            // clipboard was closed

                            break;
                        }
                    },
                    ProcessState::GettingLongValue | ProcessState::ClipboardChanged => {
                        self.send_get_req();

                        continue;
                    }
                    ProcessState::SkipEvent => continue,
                },
                None => {
                    eprintln!("[rclip] X connection broken");
                    std::process::exit(0);
                },
            };
        }

        let tg_name = self.targets.get_current().get_name();
        // reached last target_name, so don't catch any of declared targets (string or img)
        if tg_name.eq("TARGETS") {
            Err(GetterError::UnknownTarget)
        } else {
            Ok(tg_name)
        }
    }
}
