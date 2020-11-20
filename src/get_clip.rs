use crate::intern_atom;
use crate::Targets;
use xcb::base::Event;
use xcb::ffi::base::xcb_generic_event_t;
use xcb::Atom;
use xcb::Connection;
use xcb::Window;

const POLL_DURATION: u64 = 50;

pub struct Getter<'a> {
    connection: Connection,
    window: Window,
    clipboard_type: Atom,
    property: Atom,
    // target is type of returned result. May be text, image (png, jpeg, etc), or any other for
    // example like libreoffice formatted text
    targets: Targets<'a>,
}

pub enum EventError {
    WrongTarget,
    NotYetImplemented,
}

impl Getter<'_> {
    pub fn new() -> Self {
        let (connection, screen) = xcb::Connection::connect(None).unwrap();
        let window = connection.generate_id();
        {
            let screen = connection
                .get_setup()
                .roots()
                .nth(screen as usize)
                .ok_or(xcb::base::ConnError::ClosedInvalidScreen)
                .unwrap();

            xcb::create_window(
                &connection,
                xcb::COPY_FROM_PARENT as u8,
                window,
                screen.root(),
                0,
                0,
                1,
                1,
                0,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual(),
                &[(
                    xcb::CW_EVENT_MASK,
                    xcb::EVENT_MASK_STRUCTURE_NOTIFY | xcb::EVENT_MASK_PROPERTY_CHANGE,
                )],
            );
            connection.flush();
        }
        let clipboard_type = intern_atom(&connection, "CLIPBOARD");
        // TODO: rename:D
        let property = intern_atom(&connection, "RCLIP_SUCKS");

        let targets = Targets::new(&connection);

        Getter {
            connection,
            window,
            clipboard_type,
            property,
            targets,
        }
    }

    fn send_req(&self) {
        xcb::convert_selection(
            &self.connection,
            self.window,
            self.clipboard_type,
            self.targets.get_current().atom,
            self.property,
            xcb::CURRENT_TIME,
        );
        self.connection.flush();
    }

    fn process_event(
        &self,
        event: Event<xcb_generic_event_t>,
        buf: &mut Vec<u8>,
    ) -> Result<(), EventError> {
        let etype = event.response_type();

        match etype & !0x80 {
            xcb::SELECTION_NOTIFY => {
                let eve = unsafe { xcb::cast_event::<xcb::SelectionNotifyEvent>(&event) };
                let ev_prop = eve.property();

                if ev_prop == xcb::ATOM_NONE {
                    Err(EventError::WrongTarget)
                } else {
                    let reply = xcb::get_property(
                        &self.connection,
                        false,
                        self.window,
                        ev_prop,
                        xcb::ATOM_ANY,
                        buf.len() as u32,
                        std::u32::MAX, // TODO: explore how it work and how to get really long replies
                    )
                    .get_reply()
                    .unwrap();
                    // TODO: по идеи ещё нужно делать xcb::delete_property

                    // TODO: возможно ещё нужно добавить какой-то incr
                    if reply.type_() != self.targets.get_current().atom {
                        // println!("fad: {:?}", reply.type_());
                        Err(EventError::WrongTarget)
                    } else {
                        buf.extend_from_slice(reply.value());
                        println!("{:?}", buf);

                        Ok(())
                    }
                }
            }
            xcb::PROPERTY_NOTIFY => {
                println!("Not yet implemented");
                Err(EventError::NotYetImplemented)
            }
            _ => {
                unreachable!("what is it?")
            }
        }
    }

    pub fn get(&mut self, buf: &mut Vec<u8>) {
        self.targets.restore();
        self.send_req();

        loop {
            match self.connection.poll_for_event() {
                Some(event) => match self.process_event(event, buf) {
                    Result::Ok(()) => break,
                    Result::Err(EventError::WrongTarget) => match self.targets.roll_next() {
                        Ok(()) => self.send_req(),
                        Err(err) => {
                            println!("{:?}", err);

                            break;
                        }
                    },
                    _ => {}
                },
                None => {
                    // TODO: why park?
                    std::thread::park_timeout(std::time::Duration::from_millis(POLL_DURATION));
                    continue;
                }
            };
        }
    }

    pub fn get_current_target_name(&self) -> &str {
        self.targets.get_current().name
    }
}
