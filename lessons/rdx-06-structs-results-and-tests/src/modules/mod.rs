//! Building blocks for the pages. Only two groups, same as the FoxG kit:
//!
//! - `base`  : shared by everything (layout pieces, small ui parts)
//! - `games` : the game domain (what a game is, the list of games)
//!
//! `pub mod` (not just `mod`) so that code outside `modules` can reach them,
//! e.g. `crate::modules::base::ui::card::Card`.

pub mod base;
pub mod games;
