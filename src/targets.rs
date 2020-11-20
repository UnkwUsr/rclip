use crate::intern_atom;
use xcb::Atom;
use xcb::Connection;

pub struct Target<'a> {
    pub atom: Atom,
    pub name: &'a str,
}

#[derive(Debug)]
pub enum RollError {
    BoundReached
}

pub struct Targets<'a> {
    targets: Vec<Target<'a>>,
    cur_index: usize,
}

impl<'a> Target<'a> {
    pub fn new(connection: &Connection, name: &'a str) -> Self {
        let atom = intern_atom(&connection, name);

        Target { atom, name }
    }
}

impl Targets<'_> {
    pub fn new(connection: &Connection) -> Self {
        let mut targets = Vec::new();

        let utf8 = Target::new(connection, "UTF8_STRING");
        targets.push(utf8);

        let png = Target::new(connection, "image/png");
        targets.push(png);

        // let mut target = intern_atom!("TARGETS");
        // let mut target = intern_atom!("STRING");

        Targets {
            cur_index: 0,
            targets,
        }
    }

    pub fn get_current(&self) -> &Target {
        &self.targets[self.cur_index]
    }

    pub fn roll_next(&mut self) -> Result<(), RollError> {
        if self.cur_index + 1 > self.targets.len() {
            Err(RollError::BoundReached)
        } else {
            self.cur_index += 1;
            Ok(())
        }
    }

    pub fn restore(&mut self) {
        self.cur_index = 0;
    }
}
