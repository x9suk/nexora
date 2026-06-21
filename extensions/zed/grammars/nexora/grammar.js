module.exports = grammar({
  name: "nexora",

  extras: ($) => [/\s/, $.line_comment, $.block_comment],

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) =>
      choice(
        $.let_statement,
        $.func_statement,
        $.return_statement,
        $.if_statement,
        $.while_statement,
        $.for_statement,
        $.break_statement,
        $.continue_statement,
        $.print_statement,
        $.expression_statement,
        $.import_statement,
        $.class_statement,
        $.try_statement,
        $.throw_statement,
        $.assert_statement,
        $.test_statement,
        $.match_statement
      ),

    let_statement: ($) =>
      seq("let", $.identifier, optional(seq("=", $._expression)), ";"),

    func_statement: ($) =>
      seq(
        "func",
        $.identifier,
        field("parameters", $.parameter_list),
        optional(seq("->", $.type_annotation)),
        $.block
      ),

    parameter_list: $ =>
      seq("(", optional(commaSep($.parameter)), ")"),

    parameter: ($) =>
      seq($.identifier, optional(seq(":", $.type_annotation))),

    type_annotation: ($) =>
      choice(
        $.identifier,
        $.generic_type
      ),

    generic_type: ($) =>
      seq($.identifier, "<", commaSep1($.type_annotation), ">"),

    return_statement: ($) => seq("return", optional($._expression), ";"),

    if_statement: ($) =>
      prec.right(
        seq(
          "if",
          field("condition", $._expression),
          field("consequence", $.block),
          optional(seq("else", field("alternative", choice($.block, $.if_statement))))
        )
      ),

    while_statement: ($) =>
      seq("while", $._expression, $.block),

    for_statement: ($) =>
      seq(
        "for",
        $.identifier,
        "in",
        $._expression,
        $.block
      ),

    break_statement: ($) => "break",

    continue_statement: ($) => "continue",

    print_statement: ($) => seq("print", "(", optional(commaSep($._expression)), ")", ";"),

    expression_statement: ($) => seq($._expression, ";"),

    import_statement: ($) =>
      seq(
        "import",
        choice(
          seq("{", commaSep($.import_specifier), "}", "from", $.string),
          $.identifier,
          seq($.identifier, "from", $.string)
        ),
        ";"
      ),

    import_specifier: ($) =>
      seq($.identifier, optional(seq("as", $.identifier))),

    class_statement: ($) =>
      seq(
        "class",
        $.identifier,
        optional(seq("extends", $.identifier)),
        $.block
      ),

    try_statement: ($) =>
      seq(
        "try",
        $.block,
        "catch",
        optional(seq("(", $.identifier, ")")),
        $.block,
        optional(seq("finally", $.block))
      ),

    throw_statement: ($) => seq("throw", $._expression, ";"),

    assert_statement: ($) => seq("assert", "(", $._expression, ")", ";"),

    test_statement: ($) =>
      seq(
        "test",
        $.string,
        $.block
      ),

    match_statement: ($) =>
      seq(
        "match",
        $._expression,
        "{",
        repeat($.match_arm),
        "}"
      ),

    match_arm: ($) =>
      seq($._expression, "=>", $._expression, optional(",")),

    block: ($) => seq("{", repeat($._statement), "}"),

    _expression: ($) =>
      prec.left(
        choice(
          $.binary_expression,
          $.unary_expression,
          $.call_expression,
          $.member_expression,
          $.index_expression,
          $.arrow_function,
          $.parenthesized_expression,
          $.identifier,
          $.string,
          $.number,
          $.boolean,
          $.null,
          $.array,
          $.object,
          $.new_expression,
          $.this_expression,
          $.await_expression,
          $.async_expression,
          $.html_call
        )
      ),

    binary_expression: ($) =>
      prec.left(
        1,
        choice(
          seq($._expression, "+", $._expression),
          seq($._expression, "-", $._expression),
          seq($._expression, "*", $._expression),
          seq($._expression, "/", $._expression),
          seq($._expression, "%", $._expression),
          seq($._expression, "**", $._expression),
          seq($._expression, "==", $._expression),
          seq($._expression, "!=", $._expression),
          seq($._expression, "<", $._expression),
          seq($._expression, ">", $._expression),
          seq($._expression, "<=", $._expression),
          seq($._expression, ">=", $._expression),
          seq($._expression, "&&", $._expression),
          seq($._expression, "||", $._expression),
          seq($._expression, "=", $._expression),
          seq($._expression, "+=", $._expression),
          seq($._expression, "-=", $._expression)
        )
      ),

    unary_expression: ($) =>
      prec.right(
        choice(
          seq("!", $._expression),
          seq("-", $._expression)
        )
      ),

    call_expression: ($) =>
      prec.left(
        seq($._expression, "(", optional(commaSep($._expression)), ")")
      ),

    member_expression: ($) =>
      prec.left(
        seq($._expression, ".", $.identifier)
      ),

    index_expression: ($) =>
      prec.left(
        seq($._expression, "[", $._expression, "]")
      ),

    arrow_function: ($) =>
      prec.right(
        seq(
          choice(
            $.identifier,
            seq("(", optional(commaSep($.parameter)), ")")
          ),
          "=>",
          choice($._expression, $.block)
        )
      ),

    parenthesized_expression: ($) => seq("(", $._expression, ")"),

    new_expression: ($) =>
      seq("new", $.identifier, "(", optional(commaSep($._expression)), ")"),

    this_expression: ($) => "this",

    await_expression: ($) => prec(1, seq("await", $._expression)),

    async_expression: ($) => prec(1, seq("async", $._expression)),

    html_call: ($) =>
      prec.right(
        seq(
          choice(
            "div",
            "h1", "h2", "h3", "h4", "h5", "h6",
            "p", "span", "a",
            "ul", "ol", "li",
            "table", "tr", "td",
            "button", "form",
            "section", "nav", "header", "footer", "main",
            "element", "html", "render"
          ),
          "(",
          optional(commaSep($._expression)),
          ")"
        )
      ),

    array: ($) => seq("[", optional(commaSep($._expression)), "]"),

    object: ($) =>
      seq(
        "{",
        optional(commaSep($.object_field)),
        "}"
      ),

    object_field: ($) =>
      seq(
        choice($.identifier, $.string),
        ":",
        $._expression
      ),

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    string: ($) =>
      seq(
        '"',
        repeat(
          choice(
            /[^"\\$]+/,
            $.escape_sequence,
            $.interpolation
          )
        ),
        '"'
      ),

    escape_sequence: ($) =>
      /\\(?:[nrt\\\$]|u[0-9a-fA-F]{4})/,

    interpolation: ($) =>
      seq("${", $._expression, "}"),

    number: ($) => {
      const decimal = /\d+\.\d+/;
      const integer = /\d+/;
      return token(choice(decimal, integer));
    },

    boolean: ($) => token(choice("true", "false")),

    null: ($) => token("null"),

    line_comment: ($) => token(seq("//", /[^\n]*/)),

    block_comment: ($) => token(seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/")),
  },
});

function commaSep(rule) {
  return optional(commaSep1(rule));
}

function commaSep1(rule) {
  return seq(rule, repeat(seq(",", rule)));
}
