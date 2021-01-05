The following special characters must be quoted:

| & ; < > ( ) $ ` \ " ' <space> <tab> <newline>


The following may need to be quoted:

* ? [ # ˜ = %


Quotation mechanisms are single quotes, double quotes or escape character <backslash>


Single quotes preserve the literal meaning of all characters inside


Double quotes preserve the literal meaning of all characters inside, except: ` \ $

$ is for parameter expansion or arithmetic expansion. See more:
    - https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_02
    - https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_03
    - https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_04

` is for command substitution. See more: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_03

\ has special behavior

@ has special meaning


There's additional form of quoting described here: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_07_04
