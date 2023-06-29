use crate::*;

use std::ops::Range;

/// One game 'cycle'. In most packets, this will have both a tossup and a bonus,
/// but many packets are written with only tossups, and even a few out there
/// that are only bonuses.
#[derive(Clone, PartialEq, Debug)]
pub struct Cycle {
    pub tossup: Option<Tossup>,
    pub bonus: Option<Bonus>,
}

/// One quizbowl packet
#[derive(Clone, Debug)]
pub struct Packet {
    number: u8,
    description: Option<String>,
    cycles: Vec<Cycle>,
}

impl Packet {
    pub(crate) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let number = reader.read_u8()?;
        let description = {
            let str = reader.read_string()?;
            if str.len() == 0 {
                None
            } else {
                Some(str)
            }
        };
        let cycles_len = reader.read_u8()? as usize;
        let mut cycles = Vec::with_capacity(cycles_len);
        for _ in 0..cycles_len {
            let flags = reader.read_u8()?;
            let tossup = if flags & 0b10 != 0 {
                Some(Tossup::read_from(&mut reader)?)
            } else {
                None
            };
            let bonus = if flags & 0b01 != 0 {
                Some(Bonus::read_from(&mut reader)?)
            } else {
                None
            };
            cycles.push(Cycle { tossup, bonus });
        }
        Ok(Packet {
            cycles,
            description,
            number,
        })
    }

    pub(crate) fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_u8(self.number)?;
        writer.write_string(self.description.as_deref().unwrap_or(""))?;
        writer.write_u8(self.cycles.len() as u8)?;
        for cycle in &self.cycles {
            let mut flags: u8 = 0;
            if cycle.tossup.is_some() {
                flags |= 1 << 1
            }
            if cycle.bonus.is_some() {
                flags |= 1
            }
            writer.write_u8(flags)?;
            if let Some(ref tossup) = cycle.tossup {
                tossup.write_to(&mut writer)?;
            }
            if let Some(ref bonus) = cycle.bonus {
                bonus.write_to(&mut writer)?;
            }
        }
        Ok(())
    }
}

/// A pronunciation guide
#[derive(Clone, PartialEq, Debug)]
pub struct PronunciationGuide {
    /// The guide itself
    guide: String,
    /// The char index range of the text being guided
    range: Range<u16>,
}

/// Question text. Really just a string with pronunciation guides.
#[derive(Clone, PartialEq, Debug)]
pub struct QuestionText {
    raw: String,
    guides: Vec<PronunciationGuide>,
}

impl QuestionText {
    pub(crate) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let raw = reader.read_string()?;
        let guides_len = reader.read_u8()? as usize;
        let mut guides = Vec::with_capacity(guides_len);
        for _ in 0..guides_len {
            let guide = reader.read_string()?;
            let start = reader.read_u16()?;
            let end = reader.read_u16()?;
            let range = start..end;
            guides.push(PronunciationGuide { guide, range });
        }
        Ok(Self { raw, guides })
    }

    pub(crate) fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_string(&self.raw)?;
        writer.write_u8(self.guides.len() as u8)?;
        for guide in &self.guides {
            writer.write_string(&guide.guide)?;
            writer.write_u16(guide.range.start)?;
            writer.write_u16(guide.range.end)?;
        }
        Ok(())
    }
}

/// An answerline. The correct/prompt ranges are so it's a little easier to be processed by
/// something like qbreader/pkbot, it doesn't have to be perfect. Personally I've never seen any
/// other formatting besides bold/underline.
#[derive(Clone, PartialEq, Debug)]
pub struct AnswerText {
    raw: String,
    correct: Vec<Range<u8>>,
    prompt: Vec<Range<u8>>,
}

impl AnswerText {
    pub(crate) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let raw = reader.read_string()?;

        let correct_len = reader.read_u8()? as usize;
        let mut correct = Vec::with_capacity(correct_len);
        for _ in 0..correct_len {
            correct.push(reader.read_u8()?..reader.read_u8()?);
        }

        let prompt_len = reader.read_u8()? as usize;
        let mut prompt = Vec::with_capacity(prompt_len);
        for _ in 0..prompt_len {
            prompt.push(reader.read_u8()?..reader.read_u8()?);
        }

        Ok(Self {
            raw,
            correct,
            prompt,
        })
    }

    pub(crate) fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_string(&self.raw)?;
        writer.write_u8(self.correct.len() as u8)?;
        for r in &self.correct {
            writer.write_all(&[r.start, r.end])?;
        }
        writer.write_u8(self.prompt.len() as u8)?;
        for r in &self.prompt {
            writer.write_all(&[r.start, r.end])?;
        }
        Ok(())
    }
}

/// One tossup, containing one question and answer, with an optional powermark.
#[derive(Clone, PartialEq, Debug)]
pub struct Tossup {
    powermark: Option<u16>,
    second_powermark: Option<u16>,
    question: QuestionText,
    answer: AnswerText,
    category: Category,
}

impl Tossup {
    pub(crate) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        // kinda ugly but I can just cope
        let powermark = {
            let n = reader.read_u16()?;
            if n == 0 {
                None
            } else {
                Some(n)
            }
        };
        let second_powermark = if powermark.is_none() {
            None
        } else {
            let n = reader.read_u16()?;
            if n == 0 {
                None
            } else {
                Some(n)
            }
        };
        let question = QuestionText::read_from(&mut reader)?;
        let answer = AnswerText::read_from(&mut reader)?;
        let category = Category::read_from(&mut reader)?;
        Ok(Self {
            powermark,
            second_powermark,
            question,
            answer,
            category,
        })
    }

    pub(crate) fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_u16(self.powermark.unwrap_or(0))?;
        if self.powermark.is_some() {
            writer.write_u16(self.second_powermark.unwrap_or(0))?;
        }
        self.question.write_to(&mut writer)?;
        self.answer.write_to(&mut writer)?;
        self.category.write_to(&mut writer)?;
        Ok(())
    }
}

/// One part of a bonus
#[derive(Clone, PartialEq, Debug)]
pub struct BonusPart {
    value: u8,
    text: QuestionText,
    answer: AnswerText,
}

/// A bonus. Pretty self-explanitory.
#[derive(Clone, PartialEq, Debug)]
pub struct Bonus {
    leadin: QuestionText,
    category: Category,
    parts: Vec<BonusPart>,
}

impl Bonus {
    pub(crate) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let leadin = QuestionText::read_from(&mut reader)?;
        let category = Category::read_from(&mut reader)?;
        let parts_len = reader.read_u8()? as usize;
        let mut parts = Vec::with_capacity(parts_len);
        for _ in 0..parts_len {
            let value = reader.read_u8()?;
            let text = QuestionText::read_from(&mut reader)?;
            let answer = AnswerText::read_from(&mut reader)?;
            parts.push(BonusPart {
                value,
                text,
                answer,
            });
        }
        Ok(Self {
            leadin,
            category,
            parts,
        })
    }

    pub(crate) fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        self.leadin.write_to(&mut writer)?;
        self.category.write_to(&mut writer)?;
        writer.write_u8(self.parts.len() as u8)?;
        for part in &self.parts {
            writer.write_u8(part.value)?;
            part.text.write_to(&mut writer)?;
            part.answer.write_to(&mut writer)?;
        }
        Ok(())
    }
}
