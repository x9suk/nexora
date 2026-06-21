;;; nexora-mode.el --- Major mode for Nexora language -*- lexical-binding: t; -*-

;; Author: Nexora Contributors
;; Version: 1.0.0
;; Keywords: languages, nexora
;; URL: https://github.com/nexora-lang/nexora

;;; Commentary:

;; Provides syntax highlighting and basic editing support for Nexora (.nx) files.
;; Features include font-lock keywords, comments, strings, and operators.

;;; Code:

(require 'font-lock)

(defvar nexora-mode-syntax-table
  (let ((table (make-syntax-table)))
    ;; Line comments
    (modify-syntax-entry ?/ ". 12b" table)
    (modify-syntax-entry ?\n "> b" table)
    ;; Block comments
    (modify-syntax-entry ?* ". 23b" table)
    (modify-syntax-entry ?/ ". 23a" table)
    ;; Strings
    (modify-syntax-entry ?\" "\"" table)
    table)
  "Nexora mode syntax table.")

(defvar nexora-keywords
  '("let" "func" "return" "if" "else" "while" "for" "in"
    "break" "continue" "true" "false" "null" "import" "from"
    "class" "new" "this" "extends" "super" "try" "catch"
    "finally" "throw" "assert" "test" "async" "await" "match")
  "Nexora language keywords.")

(defvar nexora-builtin-functions
  '("print" "input" "type_of" "str" "int" "float" "len"
    "push" "pop" "sort" "reverse" "unique" "flatten" "range"
    "zip" "keys" "values" "entries" "map" "filter" "reduce"
    "read_file" "write_file" "json_parse" "json_stringify"
    "http_get" "http_post" "sqrt" "pow" "abs" "floor" "ceil"
    "round" "min" "max" "random" "now" "timestamp" "env"
    "exec_command" "sleep" "assert" "exists")
  "Nexora built-in functions.")

(defvar nexora-html-helpers
  '("div" "h1" "h2" "h3" "h4" "h5" "h6" "p" "span"
    "a" "ul" "ol" "li" "table" "tr" "td" "button"
    "form" "section" "nav" "header" "footer" "main"
    "element" "html" "render")
  "Nexora HTML helper functions.")

(defvar nexora-operators
  '("+" "-" "*" "/" "%" "**" "==" "!=" "<" ">" "<=" ">="
    "&&" "||" "!" "=" "=>")
  "Nexora operators.")

(defvar nexora-font-lock-keywords
  `(
    ;; Keywords
    (,(regexp-opt nexora-keywords 'symbols) . font-lock-keyword-face)
    ;; Built-in functions
    (,(regexp-opt nexora-builtin-functions 'symbols) . font-lock-builtin-face)
    ;; HTML helpers
    (,(regexp-opt nexora-html-helpers 'symbols) . font-lock-function-name-face)
    ;; Strings with interpolation
    ("\"[^\"\\\\]*\\\\.[^\"\\\\]*\"" . font-lock-string-face)
    ;; Numbers
    ("\\b[0-9]+\\.?[0-9]*\\b" . font-lock-constant-face)
    ;; Function definitions
    ("\\bfunc\\s-+\\([a-zA-Z_][a-zA-Z0-9_]*\\)" 1 font-lock-function-name-face)
    ;; Variable assignments
    ("\\blet\\s-+\\([a-zA-Z_][a-zA-Z0-9_]*\\)" 1 font-lock-variable-name-face)
    ;; Comments
    ("//.*" . font-lock-comment-face)
    ("/\\*.*?\\*/" . font-lock-comment-face))
  "Font-lock keywords for Nexora mode.")

;;;###autoload
(define-derived-mode nexora-mode prog-mode "Nexora"
  "Major mode for editing Nexora files."
  :syntax-table nexora-mode-syntax-table
  (setq font-lock-defaults '(nexora-font-lock-keywords))
  (setq comment-start "// ")
  (setq comment-end "")
  (setq comment-start-skip "//\\s-*")
  (setq indent-tabs-mode nil)
  (setq tab-width 4))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.nx\\'" . nexora-mode))

(provide 'nexora-mode)

;;; nexora-mode.el ends here
