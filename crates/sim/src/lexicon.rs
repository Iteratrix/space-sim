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

/// The canonical ladder, in the order substitutions are applied. Content writes the
/// acronym register; each entry moves a term one rung when its trigger has fired:
///
/// | content writes | act-1 slang (`first_convoy`) | Voidborn (its §6 trigger) |
/// |---|---|---|
/// | mission month, MM | count | — |
/// | the RSW | the window | the convoy (`first_silence`) |
/// | a missed RSW | a missed window | a Silence (`first_silence`) |
/// | the KEEP | the Keep | — |
/// | the MDLS | the driver | the throw (`first_silence`) |
/// | the RJSA | the collar | — |
/// | N2 make-up | the leak | the Leak (`first_silence`) |
/// | the SMB | the board | the ring (`first_silence`) |
/// | CCI | morale | grievance (`first_silence`) |
/// | SOS | the audit line | suspicion (`first_silence`) |
/// | CED | dose | tithe (`dose_ledger`) |
/// | LSS-C | closure | — |
/// | EVA | going outside | — (the Voiding is the rite, `first_voiding`) |
/// | SMR | the reset | the blanking (`unforgetting`) |
/// | the STE | the flare | the Burning (`first_voiding`) |
/// | the Sun | — | the Light (`reactor_dead`) |
/// | radiation | — | the Dark (`first_voiding`) |
/// | the sponsor's seat | the sponsor's chair | the Silence (`first_silence`) |
/// | the drum crew / the surface crew | burrowers / hull crews | the Kept / the Thin (`estates_named`) |
/// | the anniversary | the festival | the Still (`first_midwinter`) |
/// | the deceased | the dead | the returning (`first_line_launch`) |
/// | the walk outside | — | the Voiding (`first_voiding`) |
/// | not knowing who is in charge | — | grace (`licence_grace`) |
pub const WORDS: &[Word] = &[
    Word {
        old: "mission months",
        new: "counts",
        trigger: "first_convoy",
    },
    Word {
        old: "mission month",
        new: "count",
        trigger: "first_convoy",
    },
    Word {
        old: "MM",
        new: "count",
        trigger: "first_convoy",
    },
    Word {
        old: "the RSWs",
        new: "the windows",
        trigger: "first_convoy",
    },
    Word {
        old: "the RSW",
        new: "the window",
        trigger: "first_convoy",
    },
    Word {
        old: "an RSW",
        new: "a window",
        trigger: "first_convoy",
    },
    Word {
        old: "a missed RSW",
        new: "a missed window",
        trigger: "first_convoy",
    },
    Word {
        old: "the Earth window",
        new: "the convoy",
        trigger: "first_silence",
    },
    Word {
        old: "missed windows",
        new: "Silences",
        trigger: "first_silence",
    },
    Word {
        old: "missed window",
        new: "Silence",
        trigger: "first_silence",
    },
    Word {
        old: "the KEEP",
        new: "the Keep",
        trigger: "first_convoy",
    },
    Word {
        old: "KEEP",
        new: "Keep",
        trigger: "first_convoy",
    },
    Word {
        old: "the MDLS",
        new: "the driver",
        trigger: "first_convoy",
    },
    Word {
        old: "MDLS",
        new: "driver",
        trigger: "first_convoy",
    },
    Word {
        old: "the driver",
        new: "the throw",
        trigger: "first_silence",
    },
    Word {
        old: "the mass driver",
        new: "the throw",
        trigger: "first_silence",
    },
    Word {
        old: "the RJSA",
        new: "the collar",
        trigger: "first_convoy",
    },
    Word {
        old: "RJSA",
        new: "collar",
        trigger: "first_convoy",
    },
    Word {
        old: "the N2 make-up",
        new: "the leak",
        trigger: "first_convoy",
    },
    Word {
        old: "N2 make-up",
        new: "the leak",
        trigger: "first_convoy",
    },
    Word {
        old: "months",
        new: "counts",
        trigger: "first_convoy",
    },
    Word {
        old: "month",
        new: "count",
        trigger: "first_convoy",
    },
    Word {
        old: "the leak",
        new: "the Leak",
        trigger: "first_silence",
    },
    Word {
        old: "the SMB",
        new: "the board",
        trigger: "first_convoy",
    },
    Word {
        old: "SMB",
        new: "board",
        trigger: "first_convoy",
    },
    Word {
        old: "the board",
        new: "the ring",
        trigger: "first_silence",
    },
    Word {
        old: "CCI",
        new: "morale",
        trigger: "first_convoy",
    },
    Word {
        old: "morale",
        new: "grievance",
        trigger: "first_silence",
    },
    Word {
        old: "SOS",
        new: "the audit line",
        trigger: "first_convoy",
    },
    Word {
        old: "the audit line",
        new: "suspicion",
        trigger: "first_silence",
    },
    Word {
        old: "CED",
        new: "dose",
        trigger: "first_convoy",
    },
    Word {
        old: "cumulative dose",
        new: "tithe",
        trigger: "dose_ledger",
    },
    Word {
        old: "LSS-C",
        new: "closure",
        trigger: "first_convoy",
    },
    Word {
        old: "EVA",
        new: "going outside",
        trigger: "first_convoy",
    },
    Word {
        old: "the SMR",
        new: "the reset",
        trigger: "first_convoy",
    },
    Word {
        old: "SMR",
        new: "reset",
        trigger: "first_convoy",
    },
    Word {
        old: "the reset",
        new: "the blanking",
        trigger: "unforgetting",
    },
    Word {
        old: "the STE",
        new: "the flare",
        trigger: "first_convoy",
    },
    Word {
        old: "an STE",
        new: "a flare",
        trigger: "first_convoy",
    },
    Word {
        old: "the flare",
        new: "the Burning",
        trigger: "first_voiding",
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
        old: "the sponsor's seat",
        new: "the sponsor's chair",
        trigger: "first_convoy",
    },
    Word {
        old: "the sponsor's chair",
        new: "the Silence",
        trigger: "first_silence",
    },
    Word {
        old: "the drum crew",
        new: "burrowers",
        trigger: "first_convoy",
    },
    Word {
        old: "the surface crew",
        new: "hull crews",
        trigger: "first_convoy",
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
        old: "the anniversary",
        new: "the festival",
        trigger: "first_convoy",
    },
    Word {
        old: "the festival",
        new: "the Still",
        trigger: "first_midwinter",
    },
    Word {
        old: "the deceased",
        new: "the dead",
        trigger: "first_convoy",
    },
    Word {
        old: "the dead",
        new: "the returning",
        trigger: "first_line_launch",
    },
    Word {
        old: "the walk outside",
        new: "the Voiding",
        trigger: "first_voiding",
    },
    Word {
        old: "not knowing who is in charge",
        new: "grace",
        trigger: "licence_grace",
    },
    Word {
        old: "day",
        new: "watch",
        trigger: "first_silence",
    },
    Word {
        old: "days",
        new: "watches",
        trigger: "first_silence",
    },
];

/// Renders text through the words whose triggers have fired. Matches whole words only.
#[must_use]
pub fn render(game: &Game, text: &str) -> String {
    render_with(&game.lexicon_triggers, text)
}

/// Renders text through a given set of fired triggers (a chronicle entry's snapshot).
#[must_use]
pub fn render_with(triggers: &std::collections::BTreeSet<String>, text: &str) -> String {
    let mut out = text.to_owned();
    for Word { old, new, trigger } in WORDS {
        if triggers.contains(*trigger) {
            out = replace_word(&out, old, new);
        }
    }
    out
}

/// Renders a chronicle entry in the vocabulary of its own count.
#[must_use]
pub fn render_entry(entry: &crate::state::ChronicleEntry) -> String {
    render_with(&entry.words, &entry.text)
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn replace_word(text: &str, old: &str, new: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find(old) {
        let before_ok = rest[..pos]
            .chars()
            .next_back()
            .is_none_or(|c| !is_word_char(c));
        let after_ok = rest[pos + old.len()..]
            .chars()
            .next()
            .is_none_or(|c| !is_word_char(c));
        out.push_str(&rest[..pos]);
        if before_ok && after_ok {
            out.push_str(new);
        } else {
            out.push_str(old);
        }
        rest = &rest[pos + old.len()..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::replace_word;

    #[test]
    fn whole_words_only() {
        assert_eq!(
            replace_word("the day today", "day", "watch"),
            "the watch today"
        );
        assert_eq!(
            replace_word("days and day", "day", "watch"),
            "days and watch"
        );
    }
}
