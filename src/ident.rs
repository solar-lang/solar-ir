
/// Absolute Identifier
#[derive(Eq, PartialEq, Hash)]
pub struct AbsIdent {
    pub path: Vec<String>,
    pub generics: Vec<AbsIdent>,
}

pub type TypeIdent = AbsIdent;

impl std::fmt::Debug for AbsIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.join("."))?;

        if self.generics.len() > 0 {
            write!(f, "<")?;
            for (i, ident) in self.generics.iter().enumerate() {
                write!(f, "{:?}", ident)?;

                if i+1 == self.generics.len() {
                    break;
                }

                write!(f, ",")?;
            }
            write!(f, ">")?;
        }

        Ok(())
    }
}

