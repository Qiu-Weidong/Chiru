grammar syntaxis;

// parser_rule_list: parser_rule*;
rule_list: (parser_rule | lexer_rule)*;

// 语法规则
parser_rule: RULE_REF COLON block SEMI;
block: alternative (OR alternative)*;

alternative: element+ | epsilon;
epsilon: EPSILON;
element: (
		TOKEN_REF
		| STRING_LITERAL
		| RULE_REF
		| LPAREN block RPAREN
	) ebnf_suffix?;

ebnf_suffix: (STAR | PLUS | QUESTION) QUESTION?;

// 词法规则 (暂时先直接使用正则表达式)
lexer_rule: TOKEN_REF COLON regular SEMI;
regular: REGULAR_LITERAL;

/*
  rule_list: (parser_rule | lexer_rule)*;

  parser_rule: RULE_REF COLON block SEMI;
  block: alternative (OR alternative)*;

  alternative: element+ | epsilon;
  epsilon: EPSILON;
  element: (
      TOKEN_REF
      | STRING_LITERAL
      | RULE_REF
      | LPAREN block RPAREN
    ) ebnf_suffix?;

  ebnf_suffix: (STAR | PLUS | QUESTION) QUESTION?;


  lexer_rule: TOKEN_REF COLON regular SEMI;
  regular: REGULAR_LITERAL;

  RULE_REF: /[a-z][a-zA-Z0-9_]+/;
  TOKEN_REF: /[A-Z][a-zA-Z0-9_]+/;
  COLON: /::=|:=|->|=>|:|=/;
  SEMI: /;/;
  OR: /\|/;
  EPSILON: /ε|epsilon/;
  STAR: /\* /;
  PLUS: /\+/;
  QUESTION: /\?/;
  LPAREN: /\(/;
  RPAREN: /\)/;
  STRING_LITERAL: /"([^\a\d\n\r\t\f\v\\"]|(\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*"/;
  REGULAR_LITERAL: /\/([^/]|\\\/)+\//;


 对产生式需要进行改写，然后分析
 E -> ... T* ...; 需要添加 E -> ... T1 ...; T1 -> T T1 | epsilon;
 E -> ... T+ ...; 改写为 E -> ... T2 ...; T2 -> T T2 | T;
 E -> ... T? ...; 改写为 E -> ... T3 ...; T3 -> T | epsilon;
 
 
 E -> ... T* ...; => E -> ... T1 ...; T1 -> T T1 | epsilon;
 E -> ... T*? ...; => E -> ... T1 ...; T1 -> epsilon | T T1 ;
 
 E -> ... T+ ...; => E -> ... T1 ...; T1 -> T T1 | T;
 E -> ... T+? ...; => E -> ... T1 ...; T1 -> T | T T1 ;
 
 E -> ... T? ...; => E -> ... T1 ...; T1 -> T | epsilon;
 E -> ... T?? ...; => E -> ... T1 ...; T1 -> epsilon | T ;
 
 */

LINE_COMMENT: '//' .*? '\r'? '\n' -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
WS: [ \t\r\n]+ -> skip;
FRAGMENT: 'fragment';

TOKEN_REF:
	'A' ..'Z' ('a' ..'z' | 'A' ..'Z' | '0' ..'9' | '_')*;
RULE_REF:
	'a' ..'z' ('a' ..'z' | 'A' ..'Z' | '0' ..'9' | '_')*;
// LEXER_CHAR_SET: /* todo */;

COLON: Colon;
OR: Pipe;
fragment Colon: ':';
SEMI: Semi;
fragment Semi: ';';
fragment Pipe: '|';
fragment Question: '?';
fragment Star: '*';
STAR: Star;
PLUS: Plus;
QUESTION: Question;
DOT: Dot;
fragment Dot: '.';
fragment Plus: '+';
NOT: Tilde;
fragment Tilde: '~';
LPAREN: LParen;
LParen: '(';
RPAREN: RParen;
RParen: ')';
POUND: Pound;
fragment Pound: '#';
STRING_LITERAL: SQuoteLiteral;
fragment SQuoteLiteral:
	SQuote (EscSeq | ~ ('\'' | '\r' | '\n' | '\\'))* SQuote;
fragment SQuote: '\'';
fragment EscSeq:
	Esc (
		('b' | 't' | 'n' | 'f' | 'r' | '"' | '\'' | '\\')
		| UnicodeEsc
		| .
		| EOF
	);
fragment Esc: '\\';
fragment UnicodeEsc:
	'u' (HexDigit (HexDigit (HexDigit HexDigit?)?)?)?;
fragment HexDigit: '0' ..'9' | 'a' ..'f' | 'A' ..'F';
RANGE: Range;
fragment Range: '..';
fragment RegularLiteral: '/' (Esc '/' | ~('/'))* '/';
REGULAR_LITERAL: RegularLiteral;
fragment Epsilon: 'epsilon';
EPSILON: 'ε' | Epsilon;

