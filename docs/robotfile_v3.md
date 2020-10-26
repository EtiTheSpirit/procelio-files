# The RobotFile File Format: Version `3`

This is a proposed alteration of robot files. It is not intended for production use at this time and is more comparable to a crude spitball alternative.

All multi-byte numbers are big-endian and unsigned unless otherwise mentioned.
All fields with `?` after them may not be included. The corresponding type has information on how to handle that.

### Header Region
```
4 bytes: magic number (0xC571B040)
1 byte: version (3)*
8 bytes: metadata

1 byte: length of bot name
 N bytes: bot name (bytes comprising UTF8 name)

4 bytes: num parts
 K parts
 
4 bytes: num variant assignments
 I variants

16 bytes: MD5 hash from byte 8 (after #/version) to just before the hash
```
###### * FUTURE IN MIND: Version will be written UP TO 254. For forward compatibility, if we somehow beat the odds and need more than 254 versions, VERSION=0xFF will signify that the version number takes *two bytes* instead of one. 
###### Basically, <255=just use that byte for the version, 255=get the second byte + 255 for the version.
###### 0xFE = Version 254, 0xFF 0x00 = Version 255, 0xFF 0x01 = Version 256, 0xFF 0x02 = Version 257, ...
###### No cases where this would actually be employed are immediately visible, but having that just-in-case is fine.

### Part
```
4 bytes: block type
1 byte SIGNED: x position
1 byte SIGNED: y position
1 byte SIGNED: z position
1 byte: rotation
2 bytes: [15 bit color]*
1 byte: length of "extra data" region
N bytes: "extra data" (game-defined)
```
###### * Color will leave the straggler 16th bit out. It could also be used for extended behavior.

### Variant
```
4 bytes: target block index
1 byte: variant ID*
1 byte: variant data length*
 N bytes: arbitrary variant data
```
###### * MAYBE could use a pair of half-bytes here (upper half = variant ID, lower half = # of bytes for arbitrary data). Variant size is pretty small anyway. I dunno.

### JSON
New JSON should follow this form. The following fields 
```
{
  "name": "robot",
  "metadata": 3,
  "parts": [
    {
      "id": 32,
      "flags": 5,
      "variant": 0,
      "pos": [1, 4, -1],
      "rot": 68,
      "color": [255, 255, 0],
      "alpha": 255,
      "extra_data": []
    },
    "variants": [
       "target": 0
       "id": 1,
       "parameters": [
         {data...},
         {data...},
       ]
    ],
    ...
  ]
}
```
