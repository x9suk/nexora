package nexora;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.tree.IElementType;

%%

%class NexoraLexer
%implements FlexLexer
%unicode
%function advance
%type IElementType

%{
    private boolean inBlockComment = false;
%}

LINE_COMMENT=("//" .* | "//"[^\n]*)
BLOCK_COMMENT_START="/*"
BLOCK_COMMENT_END="*/"

STRING="\"" ([^\"\\] | "\\\\" | "\\\"" | "\\n" | "\\t" | "${" [^}]* "}")* "\""

INTEGER=[0-9]+
FLOAT=[0-9]+ "." [0-9]+

OPERATOR=("+" | "-" | "*" | "/" | "%" | "**" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" | "!" | "=" | "=>")

IDENTIFIER=[a-zA-Z_][a-zA-Z0-9_]*
WHITESPACE=[ \t\n]+

%state BLOCK_COMMENT

%%

<YYINITIAL> {
    {LINE_COMMENT} { return NexoraTokenTypes.COMMENT; }
    {BLOCK_COMMENT_START} { yybegin(BLOCK_COMMENT); return NexoraTokenTypes.COMMENT; }
    {STRING} { return NexoraTokenTypes.STRING; }
    {FLOAT} { return NexoraTokenTypes.NUMBER; }
    {INTEGER} { return NexoraTokenTypes.NUMBER; }
    {OPERATOR} { return NexoraTokenTypes.OPERATOR; }
    {WHITESPACE} { return NexoraTokenTypes.WHITE_SPACE; }

    "let" | "func" | "return" | "if" | "else" | "while" | "for" | "in" |
    "break" | "continue" | "true" | "false" | "null" | "import" | "from" |
    "class" | "new" | "this" | "extends" | "super" | "try" | "catch" |
    "finally" | "throw" | "assert" | "test" | "async" | "await" | "match" {
        return NexoraTokenTypes.KEYWORD;
    }

    "print" | "input" | "type_of" | "str" | "int" | "float" | "len" |
    "push" | "pop" | "sort" | "reverse" | "unique" | "flatten" | "range" |
    "zip" | "keys" | "values" | "entries" | "map" | "filter" | "reduce" |
    "read_file" | "write_file" | "json_parse" | "json_stringify" |
    "http_get" | "http_post" | "sqrt" | "pow" | "abs" | "floor" | "ceil" |
    "round" | "min" | "max" | "random" | "now" | "timestamp" | "env" |
    "exec_command" | "sleep" | "assert" | "exists" {
        return NexoraTokenTypes.BUILTIN;
    }

    "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "span" |
    "a" | "ul" | "ol" | "li" | "table" | "tr" | "td" | "button" |
    "form" | "section" | "nav" | "header" | "footer" | "main" |
    "element" | "html" | "render" {
        return NexoraTokenTypes.HTML_HELPER;
    }

    {IDENTIFIER} { return NexoraTokenTypes.IDENTIFIER; }
}

<BLOCK_COMMENT> {
    {BLOCK_COMMENT_END} { yybegin(YYINITIAL); return NexoraTokenTypes.COMMENT; }
    . { return NexoraTokenTypes.COMMENT; }
}
