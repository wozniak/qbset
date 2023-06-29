use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Category {
    Subcategory(Subcategory),
    Custom(u8),
}

impl Category {
    pub fn to_string(&self, customs: &[CustomCategory]) -> String {
        unimplemented!()
    }

    pub(crate) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let index = reader.read_u8()?;
        if let Ok(subcategory) = Subcategory::try_from(index) {
            Ok(Category::Subcategory(subcategory))
        } else {
            Ok(Category::Custom(index - Subcategory::VARIANTS))
        }
    }

    pub(crate) fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        match self {
            Category::Custom(i) => writer.write_u8(*i + Subcategory::VARIANTS)?,
            Category::Subcategory(i) => writer.write_u8(*i as u8)?,
        }
        Ok(())
    }
}

/// A general category of a tossup or bonus. This gives the general idea of what a
/// question is about vs the more specific `Subcategory`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BroadCategory {
    Literature,
    History,
    Science,
    Arts,
    /// Religion, Mythology, Philosophy, Social Science
    Rmpss,
    Geography,
    Other,
    Trash,
}

impl BroadCategory {
    pub fn as_subcat_other(&self) -> Subcategory {
        use BroadCategory::*;
        match *self {
            Literature => Subcategory::OtherLit,
            History => Subcategory::OtherHist,
            Arts => Subcategory::OtherFineArts,
            Rmpss => Subcategory::OtherRmpss,
            Geography => Subcategory::Geography,
            Other => Subcategory::OtherAcademic,
            Trash => Subcategory::Trash,
            Science => Subcategory::OtherSci,
        }
    }
}

/// A set's custom category.
#[derive(Clone, PartialEq, Debug)]
pub struct CustomCategory {
    pub name: String,
    pub general: BroadCategory,
}

/// The more specific category for a Bonus/Tossup
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Subcategory {
    AmericanLit = 0,
    BritishLit,
    ClassicalLit,
    EuropeanLit,
    WorldLit,
    OtherLit,
    AmericanHist,
    AncientHist,
    EuropeanHist,
    WorldHist,
    OtherHist,
    Biology,
    Chemistry,
    Physics,
    Math,
    Astronomy,
    EarthSci,
    ComputerSci,
    OtherSci,
    VisualFineArts,
    AuditoryFineArts,
    OtherFineArts,
    Religion,
    Mythology,
    Philosophy,
    SocialScience,
    /// Religion, mythology, philosophy, and social science
    OtherRmpss,
    Geography,
    OtherAcademic,
    Trash,
}

impl Subcategory {
    const VARIANTS: u8 = 31;

    /// What broad category does this fall under?
    pub fn broad_category(&self) -> BroadCategory {
        use Subcategory::*;
        match *self {
            AmericanLit | ClassicalLit | BritishLit | EuropeanLit | WorldLit | OtherLit => {
                BroadCategory::Literature
            }
            AmericanHist | AncientHist | EuropeanHist | WorldHist | OtherHist => {
                BroadCategory::History
            }
            Biology | Chemistry | Math | Physics | Astronomy | EarthSci | ComputerSci
            | OtherSci => BroadCategory::Science,
            VisualFineArts | AuditoryFineArts | OtherFineArts => BroadCategory::Arts,
            Religion | Mythology | Philosophy | SocialScience | OtherRmpss => BroadCategory::Rmpss,
            Geography => BroadCategory::Geography,
            OtherAcademic => BroadCategory::Other,
            Trash => BroadCategory::Trash,
        }
    }
}

impl TryFrom<u8> for Subcategory {
    type Error = crate::Error;
    fn try_from(value: u8) -> Result<Self> {
        if value <= 29 {
            // SAFETY: checked against enum variant count to make sure that this is a valid
            // variant
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(crate::Error::MalformedInput(format!(
                "{} does not fit in the subcategory enum",
                value
            )))
        }
    }
}
