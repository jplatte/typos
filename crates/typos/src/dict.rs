use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Status<'c> {
    Valid,
    Invalid,
    Corrections(Vec<Cow<'c, str>>),
}

impl<'c> Status<'c> {
    pub fn is_invalid(&self) -> bool {
        matches!(self, Status::Invalid)
    }
    pub fn is_valid(&self) -> bool {
        matches!(self, Status::Valid)
    }
    pub fn is_correction(&self) -> bool {
        matches!(self, Status::Corrections(_))
    }

    pub fn corrections_mut(&mut self) -> impl Iterator<Item = &mut Cow<'c, str>> {
        match self {
            Status::Corrections(corrections) => itertools::Either::Left(corrections.iter_mut()),
            _ => itertools::Either::Right([].iter_mut()),
        }
    }

    pub fn borrow(&self) -> Status<'_> {
        match self {
            Status::Corrections(corrections) => {
                let corrections = corrections
                    .iter()
                    .map(|c| Cow::Borrowed(c.as_ref()))
                    .collect();
                Status::Corrections(corrections)
            }
            _ => self.clone(),
        }
    }
}

pub trait Dictionary: Send + Sync {
    fn correct_ident<'s, 'w>(&'s self, _ident: crate::tokens::Identifier<'w>)
        -> Option<Status<'s>>;

    fn correct_word<'s, 'w>(&'s self, word: crate::tokens::Word<'w>) -> Option<Status<'s>>;
}