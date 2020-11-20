use xcb::Atom;
use xcb::Connection;

pub fn intern_atom (connection: &Connection, name: &str ) -> Atom {
        xcb::intern_atom(connection, false, name)
            .get_reply()
            .map(|reply| reply.atom())
            .unwrap()
}

