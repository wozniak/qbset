> This is very WIP!

# Quizbowl Set File Format (.qbset/.qbs)

## On strings
If something here says `String`, what it means is a 16-bit unsigned int that
holds the length of the string, followed by a UTF-8 byte array with said length.
There's no real reason why any string in a quizbowl packet should be more than
65,535 bytes long... that's nearly 10,000 words.

## Header
The file will begin with a header containing basic information about the packet.
```c
char[6]   fileMagic; // always "QbSet\0"
String    setName;
u16       setYear;
u8        packetCount;
Packet[]  packets;
```

## Packets
After the header, there are the actual packets. The number of packets is
determined by the `packetCount` field in the header.
```c
u8      packetIndex;
string  descriptor; // "Finals", etc.
u8      cycleCount;
Cycle[] cyles;
```

## Cycles
A cycle can have a tossup, a bonus, or both. What each specific cycle has is
defined in its `flags` a single byte. More flags may be added later.
```c
u8     flags;
Tossup tossup; // only if the flag is set!
Bonus  bonus; // ''
```
### Flags
```c
FLAG_HASBONUS  = 0b01;
FLAG_HASTOSSUP = 0b10;
```

## Tossup
coming soon
