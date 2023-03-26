grammar syntaxis;
LINE_COMMENT: '//' .*? '\r'? '\n' -> skip ;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
WS: [ \t\r\n]+ -> skip;

// token 不能依赖 token fragment 要检查依赖是否形成了环形, 然后按照拓扑排序的方式进行解析。 暂不支持 [] 这种方式。
rules: (lexerRuleSpec|parserRuleSpec)*;

lexerRuleSpec: FRAGMENT? TOKEN_REF COLON lexerRuleBlock SEMI;
parserRuleSpec: RULE_REF COLON ruleBlock SEMI;

ruleBlock: ruleAltList;
ruleAltList: labeledAlt (OR labeledAlt)*;
labeledAlt: alternative (POUND identifier)?;
identifier: RULE_REF | TOKEN_REF;
alternative: element+ |; // explicitly allow empty alts
element:
	labeledElement (ebnfSuffix |)
	| atom (ebnfSuffix |)
	| ebnf;
// | actionBlock QUESTION?;
ebnf: block blockSuffix?;

blockSuffix: ebnfSuffix;
labeledElement: (atom | block);
atom: terminal | RULE_REF | notSet | DOT;
block: LPAREN altList RPAREN;
altList: alternative (OR alternative)*;
lexerRuleBlock: lexerAltList;
lexerAltList: lexerAlt (OR lexerAlt)*;
lexerAlt: lexerElements |; // explicitly allow empty alts

lexerElements: lexerElement+ |;
lexerElement:
	lexerAtom ebnfSuffix?
	| lexerBlock ebnfSuffix?; // (xxx)*  (xxx)?   (xxx)+

lexerBlock: LPAREN lexerAltList RPAREN;

lexerAtom:
	characterRange // 'a'..'b' -> RangeSet
	| terminal
	| notSet
	// | LEXER_CHAR_SET // -> RangeSet
	| DOT; // 全集

terminal: TOKEN_REF | STRING_LITERAL;

// 返回 rangeset
characterRange: STRING_LITERAL RANGE STRING_LITERAL;

// 取反即可
notSet: NOT setElement | NOT blockSet;

// 将子元素取并集即可
blockSet: LPAREN setElement (OR setElement)* RPAREN;

// 必须返回 rangeset, 即匹配长度为 1 
setElement: // -> RangeSet
	TOKEN_REF // 只能是RangeSet这种
	| STRING_LITERAL // 只能是一个字符这种
	| characterRange;
// | LEXER_CHAR_SET;

// 后面的问号是贪婪模式, 暂时先忽略
ebnfSuffix:
	QUESTION QUESTION?
	| STAR QUESTION?
	| PLUS QUESTION?;

FRAGMENT: 'fragment';

TOKEN_REF: 'A' ..'Z' ('a'..'z' | 'A' ..'Z' | '0' ..'9' | '_')*;
RULE_REF: 'a' ..'z' ('a'..'z' | 'A' ..'Z' | '0' ..'9' | '_')*;
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
fragment SQuoteLiteral: SQuote (EscSeq | ~ ('\'' | '\r' | '\n'| '\\'))* SQuote;
fragment SQuote: '\'';
fragment EscSeq: Esc (('b'|'t'|'n'|'f'|'r'|'"'|'\''|'\\') | UnicodeEsc | . | EOF);
fragment Esc: '\\';
fragment UnicodeEsc:
	'u' (HexDigit (HexDigit (HexDigit HexDigit?)?)?)?;
fragment HexDigit: '0'..'9' | 'a'..'f' | 'A'..'F';
RANGE: Range;
fragment Range: '..';

