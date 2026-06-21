if exists('b:did_indent')
  finish
endif
let b:did_indent = 1

setlocal autoindent
setlocal tabstop=2
setlocal shiftwidth=2
setlocal softtabstop=2
setlocal expandtab
setlocal indentexpr=NexoraIndent()
setlocal indentkeys+=0},0),0],0=),0=],0=},!^F,o,O

if exists('*NexoraIndent')
  finish
endif

function! NexoraIndent() abort
  let lnum = prevnonblank(v:lnum - 1)
  if lnum == 0
    return 0
  endif

  let ind = indent(lnum)
  let prevline = getline(lnum)
  let currline = getline(v:lnum)

  " Increase indent after opening braces, parens, brackets
  if prevline =~# '[{(\[]\s*$'
    let ind += &shiftwidth
  endif

  " Decrease indent for closing braces, parens, brackets
  if currline =~# '^\s*[}\])]'
    let ind -= &shiftwidth
  endif

  " Increase indent after colon for control flow
  if prevline =~# '\v<\w+\s*\(.*\)\s*:\s*$'
    let ind += &shiftwidth
  endif

  " Indent after 'then', 'else', 'do'
  if prevline =~# '\v<func\s+.*:\s*$'
    let ind += &shiftwidth
  endif

  return ind < 0 ? 0 : ind
endfunction
