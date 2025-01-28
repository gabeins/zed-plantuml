[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "@startuml"
  "@enduml"
  "include"
] @keyword

[
  ","
] @punctuation.delimiter

[
"="
] @operator

(procedure
  (procedure_identifier) @function.call
)

(string) @string
(single_quote_string) @string
(unqouted_string) @string

(preprocessor
  url: (unqouted_string) @string.special.url
) @keyword.import

(identifier) @constant

(link) @string.special.url
