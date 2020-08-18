//! Models representing data structures from the Canvas API.

pub mod assignment;
pub mod canvas;
pub mod course;
pub mod file;
pub mod outcome;
pub mod submission;
pub mod user;

pub mod prelude {
    pub use super::assignment::*;
    pub use super::canvas::*;
    pub use super::course::*;
    pub use super::file::*;
    pub use super::outcome::*;
    pub use super::submission::*;
    pub use super::user::*;
}
