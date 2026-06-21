if exists('b:current_syntax')
  finish
endif

" Case sensitive
syn case match

" Comments
syn keyword nxTodo contained TODO FIXME NOTE HACK XXX BUG
syn region nxLineComment start='//' end='$' contains=nxTodo
syn region nxBlockComment start='/\*' end='\*/' contains=nxTodo

" Strings
syn region nxString start='"' end='"' contains=nxInterp,nxEscape skip='\\"'
syn match nxEscape contained /\\[nrt\\"]/
syn region nxInterp contained containedin=nxString start='\${' end='}' contains=nxExpr
syn region nxInterp contained containedin=nxString start='\$[a-zA-Z_]' end='\>' contains=nxExpr

" Numbers
syn match nxFloat /\v<-?\d+\.\d+>/
syn match nxInteger /\v<-?\d+>/

" Boolean and null literals
syn keyword nxBoolean true false
syn keyword nxNull null

" Keywords
syn keyword nxKeyword let func return if else while for in break continue
syn keyword nxKeyword import from class new this extends super
syn keyword nxKeyword try catch finally throw
syn keyword nxKeyword async await
syn keyword nxKeyword match

" Built-in functions
syn keyword nxBuiltin print input type_of str int float len
syn keyword nxBuiltin push pop sort reverse unique flatten
syn keyword nxBuiltin range zip keys values entries
syn keyword nxBuiltin map filter reduce
syn keyword nxBuiltin read_file write_file
syn keyword nxBuiltin json_parse json_stringify
syn keyword nxBuiltin http_get http_post
syn keyword nxBuiltin sqrt pow abs floor ceil round min max random
syn keyword nxBuiltin now timestamp env exec_command sleep
syn keyword nxBuiltin assert exists

" HTML helpers
syn keyword nxHtml div p span a ul ol li table tr td button form section nav header footer main
syn match nxHtml /\v\<(h[1-6]|element|html|render)\>/

" Operators
syn match nxOperator /[+\-*/%]/
syn match nxOperator /\*\*/
syn match nxOperator /==\|!=/
syn match nxOperator /[<>]=\?/
syn match nxOperator /&&/
syn match nxOperator /||/
syn match nxOperator /!/
syn match nxOperator /[=!]=\?/
syn match nxOperator /=/
syn match nxOperator /=>/

" Delimiters
syn match nxDelimiter /[()[\]{},;:.]/

" Function declarations
syn match nxFuncDef /\v<func\s+\zs\w+/ contained nextgroup=nxFuncParams
syn region nxFuncParams start='(' end=')' contained contains=nxParam
syn match nxParam /\v<\w+\s*(?=\=)/ contained

" Class declarations
syn match nxClassDef /\v<class\s+\zs\w+/

" Highlight links
hi def link nxTodo Todo
hi def link nxLineComment Comment
hi def link nxBlockComment Comment
hi def link nxString String
hi def link nxEscape SpecialChar
hi def link nxInterp Special
hi def link nxFloat Float
hi def link nxInteger Number
hi def link nxBoolean Boolean
hi def link nxNull Constant
hi def link nxKeyword Keyword
hi def link nxBuiltin Function
hi def link nxHtml Special
hi def link nxOperator Operator
hi def link nxDelimiter Delimiter
hi def link nxFuncDef Function
hi def link nxFuncParams Delimiter
hi def link nxParam Identifier
hi def link nxClassDef Type

let b:current_syntax = 'nexora'
