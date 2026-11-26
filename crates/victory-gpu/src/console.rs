//! Which console the title runs on. Titles pick settings (resolution,
//! memory budgets) by it, so the user chooses; Series X unless told.

use std::sync::atomic::{AtomicBool, Ordering};

const GIB: usize = 1 << 30;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Console {
    #[default]
    SeriesX,
    SeriesS,
}

impl Console {
    /// Title memory in bytes: (GPU-optimal, standard). Series X in the
    /// advanced configuration; Series S has one range, counted as standard
    /// (7,776 MiB, the published console-mode figure).
    // shortcut: later system updates gave Series S titles more (amount not
    // published here); the title sizes nothing from this figure
    // (it goes by the device type), so update it if a title does.
    pub const fn title_memory(self) -> (usize, usize) {
        match self {
            Console::SeriesX => (10 * GIB, 7 * GIB / 2),
            Console::SeriesS => (0, 7_776 << 20),
        }
    }

    /// The GPU's pipe configuration, which the addresses of the XOR swizzle
    /// modes follow (textures the title's CPU reads or writes in one).
    pub const fn pipes(self) -> victory_formats::texture::Pipes {
        match self {
            Console::SeriesX => victory_formats::texture::Pipes::SERIES_X,
            Console::SeriesS => victory_formats::texture::Pipes::SERIES_S,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Console::SeriesX => "Xbox Series X",
            Console::SeriesS => "Xbox Series S",
        }
    }
}

static SERIES_S: AtomicBool = AtomicBool::new(false);

/// Set before the title starts.
pub fn set(console: Console) {
    SERIES_S.store(console == Console::SeriesS, Ordering::Relaxed);
}

pub fn current() -> Console {
    if SERIES_S.load(Ordering::Relaxed) {
        Console::SeriesS
    } else {
        Console::SeriesX
    }
}

/// The pipe configuration console texture layouts are made with, so that
/// the pipe and bank XOR modes have addresses: the current console's, or
/// `None` with `VICTORY_XOR_LAYOUTS=0`, when the XOR modes have a size only
/// (as before their addresses were ported). Read by `crate::resource`'s
/// `texture_layout` and `victory-hle`'s `xg`.
///
/// The XOR addresses are from PastiiJay/klbf-lab ee7081b7, after
/// MiamiGame/XTP 00c60fb and 188d895, with the patterns generated out of
/// Mesa's addrlib (MIT); ported from myokex/victory-amd vi-fx3 1544bd53.
pub fn xor_pipes() -> Option<victory_formats::texture::Pipes> {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let on = *ON.get_or_init(|| {
        let on = std::env::var("VICTORY_XOR_LAYOUTS").map_or(true, |v| v.trim() != "0");
        if !on {
            tracing::info!("XOR swizzle modes without addresses (VICTORY_XOR_LAYOUTS=0)");
        }
        on
    });
    on.then(|| current().pipes())
}
