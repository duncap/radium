
" Vim syntax file
" Language: Radium
" Maintainer: Gabriel Luis
" Last Change: April 5th, 2024

if exists("b:current_syntax")
  finish
endif

" Clear old syntax to prevent conflicts
syn clear

" Comments
syn match radiumComment "##.*" display
syn match radiumComment "#.*" display
hi def link radiumComment Comment

" Strings
syn region radiumString start=+"+ end=+"+ keepend
hi def link radiumString String

" Package Declarations
syn match radiumPackage "\bpackage\s\+\w\+\b"
hi def link radiumPackage Keyword

" Control Flow
syn keyword radiumControl if else while contained
hi def link radiumControl Statement

" Function Declarations
syn match radiumFunctionDecl "\bfunc\s\+\w\+\s*("
hi def link radiumFunctionDecl Function

" Function Names
syn keyword radiumFunctionName print readFile mkdir writeFile await input true false contained
hi def link radiumFunctionName PreProc

" Variable Assignments
syn match radiumVariableAssign "\w\+\s*>>\s*"
hi def link radiumVariableAssign Operator

" Special Function Calls
syn match radiumSpecialFunctionCall "\bawait\)(\w\+)()"
hi def link radiumSpecialFunctionCall Special

" Parentheses
syn region radiumParentheses start="(" end=")" transparent contains=radiumString,radiumFunctionName
hi def link radiumParentheses Delimiter

" Function Calls
syn match radiumFunctionCall "\b\w\+\b\s*(.*)" contains=radiumFunctionName,radiumVariable
hi def link radiumFunctionCall Identifier

" Function Parameters and Variable
syn match radiumVariable "\b\w\+\b" contained
hi def link radiumVariable Identifier

let b:current_syntax = "radium"
