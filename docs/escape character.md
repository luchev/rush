Escape character <backslash> rules:

Not quoted (not escaped with <backslash> or outside of single quotes):
    if following character is newline:
        start line continuation
        when tokenizing later - remove the <backslash> <newline>
    else:
        escape the following character
Quoted (escaped with <backslash>):
    treated as normal character
