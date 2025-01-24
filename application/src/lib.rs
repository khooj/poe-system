pub mod calc_set_shared;
pub mod pipe_stashes;
mod stream;
pub mod ultimatum;

pub use stream::{open_stashes, ArchiveStashes, DirStashes, StashesIterator};
