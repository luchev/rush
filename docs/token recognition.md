<backslash> is not a token separator

if EOF
    end token

if previous character is part of an operator and current character is not quoted and current character can be used as a part of the previous operator
    append the current character to the previous operator

if previous character is part of an operator and current character cannot be used as a part of the previous operator
    end previous token

if current character is \ ' or " and is not quoted
    start quoting
    no substitttution is performed here, keep the original quoted text as is
    newline joining must be done here (this is lines ending with \ and not closing the quotes)
    quoted text is NOT token delimiter

if current character is $ or `
    mark start of parameter/arithmetic expansion or command substitution
    $ or ` are NOT token delimiters

if the curent character is not quoted and can be used as a start of a new operator
    the previous token must be delimited
    the current character is the beginning of a new token

if the current character is an unquoted whitespace
    delimit previous token
    discard the whitespace

if previous character is part of a word
    append current character to previous word

if character is #
    all following characters until newline should be discarded as a comment

the current character is consiered a start of a new word
