use super::intern_atom;
use crate::config::Config;
use xcb::Atom;
use xcb::Connection;

/// Target is a type of returned result. May be text, image (png, jpeg, etc), or any other for
/// example like libreoffice formatted text.
///
/// Xorg also provide common target named TARGETS. This target return list of all supported types
/// for current clipboard. If clipboard wiped (like when program that handled previous clipboard is
/// killed), then even TARGETS target not exist for current clipboard.
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
    pub fn new(connection: &Connection, config: &Config) -> Self {
        let mut targets = Vec::new();

        for target_name in &config.targets_list {
            let target = Target::new(connection, &target_name);
            targets.push(target);
        }

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
