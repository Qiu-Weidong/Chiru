

pub struct SyntaxisLexer;

impl SyntaxisLexer {
  pub const LINE_COMMENT: usize=1;
  pub const BLOCK_COMMENT: usize=2;
  pub const WS: usize=3;
  pub const FRAGMENT: usize=4;
  pub const TOKEN_REF: usize=5;
  pub const RULE_REF: usize=6;
  pub const COLON: usize=7;
  pub const OR: usize=8;
  pub const SEMI: usize=9;
  pub const STAR: usize=10;
  pub const PLUS: usize=11;
  pub const QUESTION: usize=12;
  pub const DOT: usize=13;
  pub const NOT: usize=14;
  pub const LPAREN: usize=15;
  pub const RPAREN: usize=16;
  pub const POUND: usize=17;
  pub const STRING_LITERAL: usize=18;
  pub const RANGE: usize=19;
  pub const REGULAR_LITERAL: usize=20;
  pub const EPSILON: usize=21;
}




