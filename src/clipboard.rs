mod ctx;
mod getter;
mod setter;
mod targets;
mod utils;

pub use ctx::ClipboardCtx;
pub use getter::Getter;
pub use setter::Setter;

use targets::Targets;
use utils::intern_atom;
