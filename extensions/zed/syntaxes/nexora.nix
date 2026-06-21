{
  "scopeName": "source.nexora",
  "fileTypes": ["nx"],
  "name": "Nexora",
  "patterns": [
    { "include": "#comments" },
    { "include": "#keywords" },
    { "include": "#constants" },
    { "include": "#strings" },
    { "include": "#numbers" },
    { "include": "#operators" },
    { "include": "#delimiters" },
    { "include": "#functions" },
    { "include": "#variables" }
  ],
  "repository": {
    "comments": {
      "patterns": [
        {
          "name": "comment.line.double-slash.nexora",
          "match": "//.*$"
        },
        {
          "name": "comment.block.nexora",
          "begin": "/\\*",
          "end": "\\*/"
        }
      ]
    },
    "keywords": {
      "patterns": [
        {
          "name": "keyword.control.nexora",
          "match": "\\b(let|func|return|if|else|while|for|in|break|continue|import|from|class|new|this|extends|super|try|catch|finally|throw|match|async|await)\\b"
        },
        {
          "name": "keyword.other.nexora",
          "match": "\\b(print|null|true|false|assert|test)\\b"
        }
      ]
    },
    "constants": {
      "patterns": [
        {
          "name": "constant.language.nexora",
          "match": "\\b(true|false|null)\\b"
        }
      ]
    },
    "strings": {
      "patterns": [
        {
          "name": "string.quoted.double.nexora",
          "begin": "\"",
          "end": "\"",
          "patterns": [
            {
              "name": "constant.character.escape.nexora",
              "match": "\\\\(?:[nrt\\\\\"$]|u[0-9a-fA-F]{4})"
            },
            {
              "name": "variable.other.interpolated.nexora",
              "begin": "\\$\\{",
              "end": "\\}",
              "patterns": [
                { "include": "source.nexora" }
              ]
            }
          ]
        }
      ]
    },
    "numbers": {
      "patterns": [
        {
          "name": "constant.numeric.float.nexora",
          "match": "\\b[0-9]+\\.[0-9]+\\b"
        },
        {
          "name": "constant.numeric.integer.nexora",
          "match": "\\b[0-9]+\\b"
        }
      ]
    },
    "operators": {
      "patterns": [
        {
          "name": "keyword.operator.arithmetic.nexora",
          "match": "\\*\\*|\\+|\\-|\\*|\\/|%"
        },
        {
          "name": "keyword.operator.comparison.nexora",
          "match": "==|!=|<=|>=|<|>"
        },
        {
          "name": "keyword.operator.logical.nexora",
          "match": "&&|\\|\\||!"
        },
        {
          "name": "keyword.operator.assignment.nexora",
          "match": "=|=>"
        }
      ]
    },
    "delimiters": {
      "patterns": [
        {
          "name": "punctuation.bracket.round.nexora",
          "match": "\\(|\\)"
        },
        {
          "name": "punctuation.bracket.curly.nexora",
          "match": "\\{|\\}"
        },
        {
          "name": "punctuation.bracket.square.nexora",
          "match": "\\[|\\]"
        },
        {
          "name": "punctuation.terminator.nexora",
          "match": ";"
        },
        {
          "name": "punctuation.separator.nexora",
          "match": ",|:"
        },
        {
          "name": "punctuation.accessor.nexora",
          "match": "\\."
        }
      ]
    },
    "functions": {
      "patterns": [
        {
          "match": "\\b([a-zA-Z_][a-zA-Z0-9_]*)\\s*(?=\\()",
          "captures": {
            "1": { "name": "entity.name.function.nexora" }
          }
        }
      ]
    },
    "variables": {
      "patterns": [
        {
          "name": "variable.other.nexora",
          "match": "\\b[a-z_][a-zA-Z0-9_]*\\b"
        },
        {
          "name": "variable.other.member.nexora",
          "match": "(?<=\\.)[a-zA-Z_][a-zA-Z0-9_]*"
        }
      ]
    }
  }
}
