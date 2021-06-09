# xml-attribute-diff
Compares all unique attribute values of two xml files. Hierarchy aswell as tag names and texts are ignored, only the values of attributes will be compared.

Usage
------------
```
xml-attribute-diff 0.1.0
Compare unique attribute values of two xml files

USAGE:
    xml-attribute-diff.exe <file1> <file2>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file1>    Original xml file, used as a reference for the comparison
    <file2>    xml file to compare to the original file
```

Example
------------
```
$xml-attribute-diff abc.xml xyz.xml
file2 has one attribute more than file1
file2 has 3 new attributes:
        blue
        red
        purple
file2 has 2 missing attributes:
        white
        black
```
abc.xml
```xml
<tag1 att1="white">
  <tag2>
    <!--Test comment-->
    Test
  </tag2>
  <tag2>
      Test 2
  </tag2>
  <tag3 desc="black"></tag3>
  <tag4 name="green"></tag4>
</tag1>
```

xyz.xml
```xml
<tag1 att1="blue">
  <tag2>
    <!--Test comment-->
    Test
  </tag2>
  <tag2>
      Test 2
  </tag2>
  <tag3 desc="red"></tag3>
  <tag3 name="purple"></tag3>
  <tag4 name="green"></tag4>
</tag1>
```
