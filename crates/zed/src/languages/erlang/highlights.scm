(module_attribute
  name: (atom) @module)

(behaviour_attribute name: (atom) @module)

(import_attribute
    module: (atom) @module)

(compile_options_attribute
    options: (tuple
      expr: (atom)
      expr: (list
        exprs: (binary_op_expr
          lhs: (atom)
          rhs: (integer)))))

(record_decl name: (atom) @type)
(record_decl name: (macro_call_expr name: (var) @constant))
(record_field name: (atom) @property)

(spec fun: (atom) @function)
(spec
  module: (module name: (atom) @module)
  fun: (atom) @function)

(callback fun: (atom) @function)


(wild_attribute name: (attr_name name: (atom) @keyword))

(pp_ifdef name: (_) @keyword.directive)
(pp_ifndef name: (_) @keyword.directive)

(pp_define
    lhs: (macro_lhs
      name: (_) @keyword.directive
      args: (var_args args: (var))))
(pp_define
    lhs: (macro_lhs
      name: (var) @constant))

(fa fun: (atom) @function)
(type_name name: (atom) @function)
(call expr: (atom) @function)
(function_clause name: (atom) @function)
(internal_fun fun: (atom) @function)

(binary_op_expr lhs: (atom) @function rhs: (integer))

(remote_module module: (atom) @module)
(remote fun: (atom) @function)
(macro_call_expr name: (var) @keyword.directive args: (_) )
(macro_call_expr name: (var) @constant)
(macro_call_expr name: (atom) @keyword.directive)
(record_field_name name: (atom) @property)
(record_name name: (atom) @type)

[ "after"
  "and"
  "band"
  "begin"
  "behavior"
  "behaviour"
  "bnot"
  "bor"
  "bsl"
  "bsr"
  "bxor"
  "callback"
  "case"
  "catch"
  "compile"
  "define"
  "div"
  "elif"
  "else"
  "end"
  "endif"
  "export"
  "export_type"
  "file"
  "fun"
  "if"
  "ifdef"
  "ifndef"
  "import"
  "include"
  "include_lib"
  "module"
  "of"
  "opaque"
  "optional_callbacks"
  "or"
  "receive"
  "record"
  "spec"
  "try"
  "type"
  "undef"
  "unit"
  "when"
  "xor"] @keyword

["andalso" "orelse"] @keyword.operator

["," "." ";"] @punctuation.delimiter
["(" ")" "{" "}" "[" "]" "<<" ">>"] @punctuation.bracket

["!"
 "->"
 "<-"
 "#"
 "::"
 "|"
 ":"
 "="
 "||"

 "+"
 "-"
 "bnot"
 "not"

 "/"
 "*"
 "div"
 "rem"
 "band"
 "and"

 "+"
 "-"
 "bor"
 "bxor"
 "bsl"
 "bsr"
 "or"
 "xor"

 "++"
 "--"

 "=="
 "/="
 "=<"
 "<"
 ">="
 ">"
 "=:="
 "=/="
 ] @operator

((var) @comment.discard
 (#match? @comment.discard "^_"))

(dotdotdot) @comment.discard
(comment) @comment

(string) @string
(char) @constant
(integer) @number
(var) @variable
(atom) @string.special.symbol