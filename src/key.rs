//! This module defines a variant of special keys and their aliases.

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    CtrlA = 0x01,
    CtrlB = 0x02,
    CtrlC = 0x03,
    CtrlD = 0x04,
    CtrlE = 0x05,
    CtrlF = 0x06,
    CtrlG = 0x07,
    CtrlH = 0x08,
    CtrlI = 0x09,
    CtrlJ = 0x0a,
    CtrlK = 0x0b,
    CtrlL = 0x0c,
    CtrlM = 0x0d,
    CtrlN = 0x0e,
    CtrlO = 0x0f,
    CtrlP = 0x10,
    CtrlQ = 0x11,
    CtrlR = 0x12,
    CtrlS = 0x13,
    CtrlT = 0x14,
    CtrlU = 0x15,
    CtrlV = 0x16,
    CtrlW = 0x17,
    CtrlX = 0x18,
    CtrlY = 0x19,
    CtrlZ = 0x1a,
    ESC = 0x1b,
    Space = 0x20,
    Backspace = 0x7f,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

pub use Key::*;

/// An alias of `Key::CtrlM`
pub const ENTER: Key = Key::CtrlM;
