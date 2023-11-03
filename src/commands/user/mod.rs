pub mod top_tracks;

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum Term {
    Short,
    Medium,
    Long,
}

impl From<Term> for crate::spotify::user::TopTerm {
    fn from(value: Term) -> Self {
        match value {
            Term::Short => Self::Short,
            Term::Medium => Self::Medium,
            Term::Long => Self::Long,
        }
    }
}
