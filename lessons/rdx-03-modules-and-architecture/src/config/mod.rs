//! App-wide constants. One place to rename the app or change a label.
//!
//! Why a module for two constants? Because "where do I change the app
//! name?" should have exactly one answer. Later this module grows
//! (storage keys, platform name...).

/// Shown in the page title and the header.
pub const APP_NAME: &str = "FoxG Game Hub";

/// Small tag in the header: which lesson this snapshot belongs to.
pub const LESSON_TAG: &str = "lesson 03";
