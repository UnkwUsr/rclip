mod ctx;
mod getter;
mod targets;
mod utils;

pub use ctx::ClipboardCtx;
pub use getter::Getter;
pub use getter::GetterError;

use targets::Targets;
use utils::intern_atom;
