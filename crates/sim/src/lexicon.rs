//! The lexicon as culture state: words enter use on triggers, and the chronicle renders through them.

use crate::state::Game;

/// One vocabulary entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    /// The old word, as written by content.
    pub old: &'static str,
    /// The Voidborn word.
    pub new: &'static str,
    /// The trigger after which the new word is used.
    pub trigger: &'static str,
}

/// The canonical table, in the order substitutions are applied.
pub const WORDS: &[Word] = &[
    Word {
        old: "month",
        new: "count",
        trigger: "first_count",
    },
    Word {
        old: "months",
        new: "counts",
        trigger: "first_count",
    },
    Word {
        old: "the Earth window",
        new: "the convoy",
        trigger: "first_convoy",
    },
    Word {
        old: "missed window",
        new: "Silence",
        trigger: "first_silence",
    },
    Word {
        old: "missed windows",
        new: "Silences",
        trigger: "first_silence",
    },
    Word {
        old: "day",
        new: "watch",
        trigger: "first_count",
    },
    Word {
        old: "days",
        new: "watches",
        trigger: "first_count",
    },
    Word {
        old: "the Sun",
        new: "the Light",
        trigger: "reactor_dead",
    },
    Word {
        old: "radiation",
        new: "the Dark",
        trigger: "first_voiding",
    },
    Word {
        old: "cumulative dose",
        new: "tithe",
        trigger: "dose_ledger",
    },
    Word {
        old: "the reset",
        new: "the blanking",
        trigger: "unforgetting",
    },
    Word {
        old: "the burrow",
        new: "the Keep",
        trigger: "first_count",
    },
    Word {
        old: "burrowers",
        new: "the Kept",
        trigger: "estates_named",
    },
    Word {
        old: "hull crews",
        new: "the Thin",
        trigger: "estates_named",
    },
    Word {
        old: "the sponsor's chair",
        new: "the Silence",
        trigger: "first_silence",
    },
    Word {
        old: "not knowing who is in charge",
        new: "grace",
        trigger: "licence_grace",
    },
    Word {
        old: "the mass driver",
        new: "the throw",
        trigger: "first_count",
    },
];

/// Renders text through the words whose triggers have fired.
#[must_use]
pub fn render(game: &Game, text: &str) -> String {
    let mut out = text.to_owned();
    for Word { old, new, trigger } in WORDS {
        if game.lexicon_triggers.contains(*trigger) {
            out = out.replace(old, new);
        }
    }
    out
}
