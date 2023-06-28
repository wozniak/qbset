> This is very WIP!

# Quizbowl Set File Format (.qbset/.qbs)

## Strings
If something here says `string`, what it means is a 16-bit unsigned int that
holds the length of the string, followed by a UTF-8 byte array with said length.
There's no real reason why any string in a quizbowl packet should be more than
65,535 bytes long... that's nearly 10,000 words of ASCII.

## Important structs
There are a few things we will use later but don't have a specific place.

#### `Range<T>`
```c
T start;
T end;
```

#### `PronounceGuide`
```c
string     guide;
Range<u16> word; // char index of start and end of thing being guided
```

#### `QuestionText`
```c
string           raw; // this does not contain any pronunciation guides
u8               nGuides;
PronounceGuide[] guides;
```

#### `AnswerText`
```c
string      raw;
u8          nCorrects;
Range<u8>[] correct; // usually things that are bolded
u8          nPrompts;
Range<u8>[] prompt; // usually things that are underlined
```

#### `Category`
General and more specific categories are not seperate, as usually it is pretty
obvious what a subcategory falls into generally. It is up to implementers to
notice this.

It's a `u8` enum, taken from qbreader.

```c
AMERICAN_LIT = 0,
BRITISH_LIT,
CLASSICAL_LIT,
EURO_LIT,
WORLD_LIT,
OTHER_LIT,
AMERICAN_HIST,
ANCIENT_HIST,
EURO_HIST,
WORLD_HIST,
OTHER_HIST,
BIOLOGY,
CHEMISTRY,
PHYSICS,
MATH,
EARTH_SCI,
COMPUTER_SCI,
OTHER_SCI,
VISUAL_FA,
AUDITORY_FA,
OTHER_FA,
RELIGION,
MYTHOLOGY,
PHILOSPHY,
SOCIAL_SCIENCE,
OTHER_RMPSS,
GEOGRAPHY,
OTHER,
TRASH
```
## Header
The file will begin with a header containing basic information about the packet.

Custom categories are an array of string, category tuples for categories that
are not included in the format. They are each the name of the category then the
category for the other section of the broader category. For example, psycology
would be `("Psychology", OTHER_SCI)`. They are referenced in questions as the
index into `customCategories` plus the number of included categories.

```c
char[6]   fileMagic; // always "QbSet\0"
string    setName;
u16       setYear;
u8        packetCount;
u8        fileVersion; // Currently 0
u8        nCustomCategories;
(string, Category)[] customCategories;
Packet[]  packets;
```

## Packets
After the header, there are the actual packets. The number of packets is
determined by the `packetCount` field in the header.
```c
u8      packetNumber;
string  descriptor; // "Finals", etc.
u8      nCycles;
Cycle[] cyles;
```

## Cycles
A cycle can have a tossup, a bonus, or both. What each specific cycle has is
defined in its `flags` a single byte. More flags may be added later.
```c
u8     flags;
Tossup tossup; // only if the flag is set!
Bonus  bonus; // ditto

```
### Flags
```c
FLAG_HASBONUS  = 0b01;
FLAG_HASTOSSUP = 0b10;
```

## Tossup
A tossup consists of one question and one answer.
```c
u16          powerMark; // The index of the first char out of power. 0 if no power.
u16          secondPm; // This field will not be included if powerMark is 0.
QuestionText question;
Category     category;
AnswerText   answer;
```

## Bonus
A bonus contains a leadin, and some amount question-answer pairs.
```c
QuestionText leadin;
Category     category;
u8           partCount;
BonusPart[]  parts;
```
#### `BonusPart`
```c
u8           value; // usually 10, but you never know
QuestionText text;
AnswerText   answer;
```
