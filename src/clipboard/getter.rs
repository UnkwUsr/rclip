use super::ClipboardCtx;
use super::Targets;

use xcb::base::Event;
use xcb::ffi::base::xcb_generic_event_t;

const LEN_PROPERTY_GET: u32 = std::u32::MAX;

pub struct Getter<'a> {
    ctx: &'a ClipboardCtx,
    targets: Targets,
    xfixes_event_base: u8,
}

pub enum ProcessState {
    Done,
    WrongTarget,
    SkipEvent,
    GettingLongValue,
    ClipboardChanged,
}

impl<'a> Getter<'a> {
    pub fn new(ctx: &'a ClipboardCtx) -> Self {
        let targets = Targets::new(&ctx.connection);

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
                    // println!("Not yet implemented");
                    ProcessState::SkipEvent
                }
                _ => {
                    unreachable!("what is it?")
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

    // will wait until clibpoard changed
    pub fn get_wait(&mut self, buf: &mut Vec<u8>) -> String {
        self.prepare_for_get();

        loop {
            match self.ctx.connection.wait_for_event() {
                Some(event) => match self.process_event(event, buf) {
                    ProcessState::Done => {
                        // don't know why, but for some applications (flameshot, for example)
                        // clipboard does not changing without deleting property
                        xcb::delete_property(
                            &self.ctx.connection,
                            self.ctx.window,
                            self.ctx.property,
                        );
                        self.ctx.connection.flush();

                        break;
                    }
                    ProcessState::GettingLongValue | ProcessState::ClipboardChanged => {
                        self.send_get_req();

                        continue;
                    }
                    ProcessState::WrongTarget => match self.targets.roll_next() {
                        Ok(()) => self.send_get_req(),
                        Err(super::targets::RollError::BoundReached) => {
                            println!("[error] can't find target of value");

                            break;
                        }
                    },
                    ProcessState::SkipEvent => continue,
                },
                None => continue,
            };
        }

        self.targets.get_current().get_name()
    }
}
