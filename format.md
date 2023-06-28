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

#### `PronunciationGuide`
```c
string     guide;
Range<u16> word; // char index of start and end of thing being guided
```

#### `QuestionText`
```c
string             raw; // this does not contain any pronunciation guides
PronunciationGuide guides;
```

#### `AnswerText`
```c
string      raw;
u8          nCorrects;
Range<u8>[] correct; // usually things that are bolded
u8          nPrompts;
Range<u8>[] prompt; // usually things that are underlined
```

## Header
The file will begin with a header containing basic information about the packet.
```c
char[6]   fileMagic; // always "QbSet\0"
string    setName;
u16       setYear;
u8        packetCount;
u8        fileVersion; // Currently 0
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
AnswerText   answer;
```

## Bonus
A bonus contains a leadin, and 3 question-answer pairs.
```c
QuestionText leadin;
u8           partCount;
BonusPart[]  parts;
```
#### `BonusPart`
```c
QuestionText text;
AnswerText answer;
```
