use super::intern_atom;

use xcb::Atom;
use xcb::Connection;

// target is type of returned result. May be text, image (png, jpeg, etc), or any other for
// example like libreoffice formatted text
pub struct Target {
    pub atom: Atom,
    pub name: String,
}

impl Target {
    pub fn new(connection: &Connection, name: &str) -> Self {
        let atom = intern_atom(&connection, name);

        Target {
            atom,
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
pub enum RollError {
    BoundReached,
}

pub struct Targets {
    targets: Vec<Target>,
    cur_index: usize,
}

impl Targets {
    pub fn new(connection: &Connection) -> Self {
        let mut targets = Vec::new();

        let utf8 = Target::new(connection, "UTF8_STRING");
        targets.push(utf8);

        let png = Target::new(connection, "image/png");
        targets.push(png);

        let request_targets_list = Target::new(connection, "TARGETS");
        targets.push(request_targets_list);

        Targets {
            cur_index: 0,
            targets,
        }
    }

    pub fn get_current(&self) -> &Target {
        &self.targets[self.cur_index]
    }

    pub fn roll_next(&mut self) -> Result<(), RollError> {
        if self.cur_index + 1 >= self.targets.len() {
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
