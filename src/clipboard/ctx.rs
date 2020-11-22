use super::intern_atom;

use xcb::Atom;
use xcb::Connection;
use xcb::Window;

pub struct ClipboardCtx {
    pub connection: Connection,
    pub window: Window,
    pub screen: i32,
    pub selection_type: Atom,
    pub property: Atom,
}

impl ClipboardCtx {
    pub fn new() -> Self {
        let (connection, screen) = xcb::Connection::connect(None).unwrap();
        let window = connection.generate_id();

        {
            let screen_ptr = connection
                .get_setup()
                .roots()
                .nth(screen as usize)
                .ok_or(xcb::base::ConnError::ClosedInvalidScreen)
                .unwrap();

            xcb::create_window(
                &connection,
                xcb::COPY_FROM_PARENT as u8,
                window,
                screen_ptr.root(),
                0,
                0,
                1,
                1,
                0,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen_ptr.root_visual(),
                &[(
                    xcb::CW_EVENT_MASK,
                    xcb::EVENT_MASK_STRUCTURE_NOTIFY | xcb::EVENT_MASK_PROPERTY_CHANGE,
                )],
            );
            connection.flush();
        }

        let selection_type = intern_atom(&connection, "CLIPBOARD");
        let property = intern_atom(&connection, "RCLIP");

        ClipboardCtx {
            connection,
            window,
            screen,
            selection_type,
            property,
        }
    }
}
