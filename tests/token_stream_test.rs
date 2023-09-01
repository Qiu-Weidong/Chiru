
// 编写一个 token_stream 的测试

use chiru::{runtime::{token_stream::TokenStream, lexer::Lexer}, tool::syntaxis::chiru_lexer::ChiruLexer};


#[test]
#[allow(unused_doc_comments)]
fn token_stream_test() {
  let input = r######"
  rule_list: (parser_rule | lexer_rule)*;

  parser_rule: RULE_REF COLON block SEMI;
  block: alternative (OR alternative)*;

  alternative: element element* | epsilon;
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

  RULE_REF: r###"[a-z][a-zA-Z0-9_]*"###;
  TOKEN_REF: r###"[A-Z][a-zA-Z0-9_]*"###;
  COLON: r###"::=|:=|->|=>|:|="###;
  SEMI: r###";"###;
  OR: r###"\|"###;
  EPSILON: r###"ε|epsilon"###;
  STAR: r###"\*"###;
  PLUS: r###"\+"###;
  QUESTION: r###"\?"###;
  LPAREN: r###"\("###;
  RPAREN: r###"\)"###;
  STRING_LITERAL: r###""((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*""###;
  REGULAR_LITERAL: r###"(?s)r##".*?"##"###;
  WHITE_SPACE: r###"[ \r\n\t\f]+"###;
  
  "######;

  /**
   * [@0, 0:0='_START', <_START>, <0>, start: <line: 0, position: 0>, stop: <line: 0, position: 0>]
   * [@1, 3:12='rule_list', <RULE_REF>, <2>, start: <line: 1, position: 2>, stop: <line: 1, position: 11>]
   * [@2, 12:13=':', <COLON>, <4>, start: <line: 1, position: 11>, stop: <line: 1, position: 12>]
   * [@3, 14:15='(', <LPAREN>, <11>, start: <line: 1, position: 13>, stop: <line: 1, position: 14>]
   * [@4, 15:26='parser_rule', <RULE_REF>, <2>, start: <line: 1, position: 14>, stop: <line: 1, position: 25>]
   * [@5, 27:28='|', <OR>, <6>, start: <line: 1, position: 26>, stop: <line: 1, position: 27>]
   * [@6, 29:39='lexer_rule', <RULE_REF>, <2>, start: <line: 1, position: 28>, stop: <line: 1, position: 38>]
   * [@7, 39:40=')', <RPAREN>, <12>, start: <line: 1, position: 38>, stop: <line: 1, position: 39>]
   * [@8, 40:41='*', <STAR>, <8>, start: <line: 1, position: 39>, stop: <line: 1, position: 40>]
   * [@9, 41:42=';', <SEMI>, <5>, start: <line: 1, position: 40>, stop: <line: 1, position: 41>]
   * [@10, 46:57='parser_rule', <RULE_REF>, <2>, start: <line: 3, position: 2>, stop: <line: 3, position: 13>]
   * [@11, 57:58=':', <COLON>, <4>, start: <line: 3, position: 13>, stop: <line: 3, position: 14>]
   * [@12, 59:67='RULE_REF', <TOKEN_REF>, <3>, start: <line: 3, position: 15>, stop: <line: 3, position: 23>]
   * [@13, 68:73='COLON', <TOKEN_REF>, <3>, start: <line: 3, position: 24>, stop: <line: 3, position: 29>]
   * [@14, 74:79='block', <RULE_REF>, <2>, start: <line: 3, position: 30>, stop: <line: 3, position: 35>]
   * [@15, 80:84='SEMI', <TOKEN_REF>, <3>, start: <line: 3, position: 36>, stop: <line: 3, position: 40>]
   * [@16, 84:85=';', <SEMI>, <5>, start: <line: 3, position: 40>, stop: <line: 3, position: 41>]
   * [@17, 88:93='block', <RULE_REF>, <2>, start: <line: 4, position: 2>, stop: <line: 4, position: 7>]
   * [@18, 93:94=':', <COLON>, <4>, start: <line: 4, position: 7>, stop: <line: 4, position: 8>]
   * [@19, 95:106='alternative', <RULE_REF>, <2>, start: <line: 4, position: 9>, stop: <line: 4, position: 20>]
   * [@20, 107:108='(', <LPAREN>, <11>, start: <line: 4, position: 21>, stop: <line: 4, position: 22>]
   * [@21, 108:110='OR', <TOKEN_REF>, <3>, start: <line: 4, position: 22>, stop: <line: 4, position: 24>]
   * [@22, 111:122='alternative', <RULE_REF>, <2>, start: <line: 4, position: 25>, stop: <line: 4, position: 36>]
   * [@23, 122:123=')', <RPAREN>, <12>, start: <line: 4, position: 36>, stop: <line: 4, position: 37>]
   * [@24, 123:124='*', <STAR>, <8>, start: <line: 4, position: 37>, stop: <line: 4, position: 38>]
   * [@25, 124:125=';', <SEMI>, <5>, start: <line: 4, position: 38>, stop: <line: 4, position: 39>]
   * [@26, 129:140='alternative', <RULE_REF>, <2>, start: <line: 6, position: 2>, stop: <line: 6, position: 13>]
   * [@27, 140:141=':', <COLON>, <4>, start: <line: 6, position: 13>, stop: <line: 6, position: 14>]
   * [@28, 142:149='element', <RULE_REF>, <2>, start: <line: 6, position: 15>, stop: <line: 6, position: 22>]
   * [@29, 150:157='element', <RULE_REF>, <2>, start: <line: 6, position: 23>, stop: <line: 6, position: 30>]
   * [@30, 157:158='*', <STAR>, <8>, start: <line: 6, position: 30>, stop: <line: 6, position: 31>]
   * [@31, 159:160='|', <OR>, <6>, start: <line: 6, position: 32>, stop: <line: 6, position: 33>]
   * [@32, 161:168='epsilon', <RULE_REF>, <2>, start: <line: 6, position: 34>, stop: <line: 6, position: 41>]
   * [@33, 168:169=';', <SEMI>, <5>, start: <line: 6, position: 41>, stop: <line: 6, position: 42>]
   * [@34, 172:179='epsilon', <RULE_REF>, <2>, start: <line: 7, position: 2>, stop: <line: 7, position: 9>]
   * [@35, 179:180=':', <COLON>, <4>, start: <line: 7, position: 9>, stop: <line: 7, position: 10>]
   * [@36, 181:188='EPSILON', <TOKEN_REF>, <3>, start: <line: 7, position: 11>, stop: <line: 7, position: 18>]
   * [@37, 188:189=';', <SEMI>, <5>, start: <line: 7, position: 18>, stop: <line: 7, position: 19>]
   * [@38, 192:199='element', <RULE_REF>, <2>, start: <line: 8, position: 2>, stop: <line: 8, position: 9>]
   * [@39, 199:200=':', <COLON>, <4>, start: <line: 8, position: 9>, stop: <line: 8, position: 10>]
   * [@40, 201:202='(', <LPAREN>, <11>, start: <line: 8, position: 11>, stop: <line: 8, position: 12>]
   * [@41, 209:218='TOKEN_REF', <TOKEN_REF>, <3>, start: <line: 9, position: 6>, stop: <line: 9, position: 15>]
   * [@42, 225:226='|', <OR>, <6>, start: <line: 10, position: 6>, stop: <line: 10, position: 7>]
   * [@43, 227:241='STRING_LITERAL', <TOKEN_REF>, <3>, start: <line: 10, position: 8>, stop: <line: 10, position: 22>]
   * [@44, 248:249='|', <OR>, <6>, start: <line: 11, position: 6>, stop: <line: 11, position: 7>]
   * [@45, 250:258='RULE_REF', <TOKEN_REF>, <3>, start: <line: 11, position: 8>, stop: <line: 11, position: 16>]
   * [@46, 265:266='|', <OR>, <6>, start: <line: 12, position: 6>, stop: <line: 12, position: 7>]
   * [@47, 267:273='LPAREN', <TOKEN_REF>, <3>, start: <line: 12, position: 8>, stop: <line: 12, position: 14>]
   * [@48, 274:279='block', <RULE_REF>, <2>, start: <line: 12, position: 15>, stop: <line: 12, position: 20>]
   * [@49, 280:286='RPAREN', <TOKEN_REF>, <3>, start: <line: 12, position: 21>, stop: <line: 12, position: 27>]
   * [@50, 291:292=')', <RPAREN>, <12>, start: <line: 13, position: 4>, stop: <line: 13, position: 5>]
   * [@51, 293:304='ebnf_suffix', <RULE_REF>, <2>, start: <line: 13, position: 6>, stop: <line: 13, position: 17>]
   * [@52, 304:305='?', <QUESTION>, <10>, start: <line: 13, position: 17>, stop: <line: 13, position: 18>]
   * [@53, 305:306=';', <SEMI>, <5>, start: <line: 13, position: 18>, stop: <line: 13, position: 19>]
   * [@54, 310:321='ebnf_suffix', <RULE_REF>, <2>, start: <line: 15, position: 2>, stop: <line: 15, position: 13>]
   * [@55, 321:322=':', <COLON>, <4>, start: <line: 15, position: 13>, stop: <line: 15, position: 14>]
   * [@56, 323:324='(', <LPAREN>, <11>, start: <line: 15, position: 15>, stop: <line: 15, position: 16>]
   * [@57, 324:328='STAR', <TOKEN_REF>, <3>, start: <line: 15, position: 16>, stop: <line: 15, position: 20>]
   * [@58, 329:330='|', <OR>, <6>, start: <line: 15, position: 21>, stop: <line: 15, position: 22>]
   * [@59, 331:335='PLUS', <TOKEN_REF>, <3>, start: <line: 15, position: 23>, stop: <line: 15, position: 27>]
   * [@60, 336:337='|', <OR>, <6>, start: <line: 15, position: 28>, stop: <line: 15, position: 29>]
   * [@61, 338:346='QUESTION', <TOKEN_REF>, <3>, start: <line: 15, position: 30>, stop: <line: 15, position: 38>]
   * [@62, 346:347=')', <RPAREN>, <12>, start: <line: 15, position: 38>, stop: <line: 15, position: 39>]
   * [@63, 348:356='QUESTION', <TOKEN_REF>, <3>, start: <line: 15, position: 40>, stop: <line: 15, position: 48>]
   * [@64, 356:357='?', <QUESTION>, <10>, start: <line: 15, position: 48>, stop: <line: 15, position: 49>]
   * [@65, 357:358=';', <SEMI>, <5>, start: <line: 15, position: 49>, stop: <line: 15, position: 50>]
   * [@66, 363:373='lexer_rule', <RULE_REF>, <2>, start: <line: 18, position: 2>, stop: <line: 18, position: 12>]
   * [@67, 373:374=':', <COLON>, <4>, start: <line: 18, position: 12>, stop: <line: 18, position: 13>]
   * [@68, 375:384='TOKEN_REF', <TOKEN_REF>, <3>, start: <line: 18, position: 14>, stop: <line: 18, position: 23>]
   * [@69, 385:390='COLON', <TOKEN_REF>, <3>, start: <line: 18, position: 24>, stop: <line: 18, position: 29>]
   * [@70, 391:398='regular', <RULE_REF>, <2>, start: <line: 18, position: 30>, stop: <line: 18, position: 37>]
   * [@71, 399:403='SEMI', <TOKEN_REF>, <3>, start: <line: 18, position: 38>, stop: <line: 18, position: 42>]
   * [@72, 403:404=';', <SEMI>, <5>, start: <line: 18, position: 42>, stop: <line: 18, position: 43>]
   * [@73, 407:414='regular', <RULE_REF>, <2>, start: <line: 19, position: 2>, stop: <line: 19, position: 9>]
   * [@74, 414:415=':', <COLON>, <4>, start: <line: 19, position: 9>, stop: <line: 19, position: 10>]
   * [@75, 416:431='REGULAR_LITERAL', <TOKEN_REF>, <3>, start: <line: 19, position: 11>, stop: <line: 19, position: 26>]
   * [@76, 431:432=';', <SEMI>, <5>, start: <line: 19, position: 26>, stop: <line: 19, position: 27>]
   * [@77, 436:444='RULE_REF', <TOKEN_REF>, <3>, start: <line: 21, position: 2>, stop: <line: 21, position: 10>]
   * [@78, 444:445=':', <COLON>, <4>, start: <line: 21, position: 10>, stop: <line: 21, position: 11>]
   * [@79, 446:466='/[a-z][a-zA-Z0-9_]+/', <REGULAR_LITERAL>, <14>, start: <line: 21, position: 12>, stop: <line: 21, position: 32>]
   * [@80, 466:467=';', <SEMI>, <5>, start: <line: 21, position: 32>, stop: <line: 21, position: 33>]
   * [@81, 470:479='TOKEN_REF', <TOKEN_REF>, <3>, start: <line: 22, position: 2>, stop: <line: 22, position: 11>]
   * [@82, 479:480=':', <COLON>, <4>, start: <line: 22, position: 11>, stop: <line: 22, position: 12>]
   * [@83, 481:501='/[A-Z][a-zA-Z0-9_]+/', <REGULAR_LITERAL>, <14>, start: <line: 22, position: 13>, stop: <line: 22, position: 33>]
   * [@84, 501:502=';', <SEMI>, <5>, start: <line: 22, position: 33>, stop: <line: 22, position: 34>]
   * [@85, 505:510='COLON', <TOKEN_REF>, <3>, start: <line: 23, position: 2>, stop: <line: 23, position: 7>]
   * [@86, 510:511=':', <COLON>, <4>, start: <line: 23, position: 7>, stop: <line: 23, position: 8>]
   * [@87, 512:530='/::=|:=|->|=>|:|=/', <REGULAR_LITERAL>, <14>, start: <line: 23, position: 9>, stop: <line: 23, position: 27>]
   * [@88, 530:531=';', <SEMI>, <5>, start: <line: 23, position: 27>, stop: <line: 23, position: 28>]
   * [@89, 534:538='SEMI', <TOKEN_REF>, <3>, start: <line: 24, position: 2>, stop: <line: 24, position: 6>]
   * [@90, 538:539=':', <COLON>, <4>, start: <line: 24, position: 6>, stop: <line: 24, position: 7>]
   * [@91, 540:543='/;/', <REGULAR_LITERAL>, <14>, start: <line: 24, position: 8>, stop: <line: 24, position: 11>]
   * [@92, 543:544=';', <SEMI>, <5>, start: <line: 24, position: 11>, stop: <line: 24, position: 12>]
   * [@93, 547:549='OR', <TOKEN_REF>, <3>, start: <line: 25, position: 2>, stop: <line: 25, position: 4>]
   * [@94, 549:550=':', <COLON>, <4>, start: <line: 25, position: 4>, stop: <line: 25, position: 5>]
   * [@95, 551:555='/\|/', <REGULAR_LITERAL>, <14>, start: <line: 25, position: 6>, stop: <line: 25, position: 10>]
   * [@96, 555:556=';', <SEMI>, <5>, start: <line: 25, position: 10>, stop: <line: 25, position: 11>]
   * [@97, 559:566='EPSILON', <TOKEN_REF>, <3>, start: <line: 26, position: 2>, stop: <line: 26, position: 9>]
   * [@98, 566:567=':', <COLON>, <4>, start: <line: 26, position: 9>, stop: <line: 26, position: 10>]
   * [@99, 568:580='/ε|epsilon/', <REGULAR_LITERAL>, <14>, start: <line: 26, position: 11>, stop: <line: 26, position: 23>]
   * [@100, 580:581=';', <SEMI>, <5>, start: <line: 26, position: 23>, stop: <line: 26, position: 24>]
   * [@101, 584:588='STAR', <TOKEN_REF>, <3>, start: <line: 27, position: 2>, stop: <line: 27, position: 6>]
   * [@102, 588:589=':', <COLON>, <4>, start: <line: 27, position: 6>, stop: <line: 27, position: 7>]
   * [@103, 590:595='/\* /', <REGULAR_LITERAL>, <14>, start: <line: 27, position: 8>, stop: <line: 27, position: 13>]
   * [@104, 595:596=';', <SEMI>, <5>, start: <line: 27, position: 13>, stop: <line: 27, position: 14>]
   * [@105, 599:603='PLUS', <TOKEN_REF>, <3>, start: <line: 28, position: 2>, stop: <line: 28, position: 6>]
   * [@106, 603:604=':', <COLON>, <4>, start: <line: 28, position: 6>, stop: <line: 28, position: 7>]
   * [@107, 605:609='/\+/', <REGULAR_LITERAL>, <14>, start: <line: 28, position: 8>, stop: <line: 28, position: 12>]
   * [@108, 609:610=';', <SEMI>, <5>, start: <line: 28, position: 12>, stop: <line: 28, position: 13>]
   * [@109, 613:621='QUESTION', <TOKEN_REF>, <3>, start: <line: 29, position: 2>, stop: <line: 29, position: 10>]
   * [@110, 621:622=':', <COLON>, <4>, start: <line: 29, position: 10>, stop: <line: 29, position: 11>]
   * [@111, 623:627='/\?/', <REGULAR_LITERAL>, <14>, start: <line: 29, position: 12>, stop: <line: 29, position: 16>]
   * [@112, 627:628=';', <SEMI>, <5>, start: <line: 29, position: 16>, stop: <line: 29, position: 17>]
   * [@113, 631:637='LPAREN', <TOKEN_REF>, <3>, start: <line: 30, position: 2>, stop: <line: 30, position: 8>]
   * [@114, 637:638=':', <COLON>, <4>, start: <line: 30, position: 8>, stop: <line: 30, position: 9>]
   * [@115, 639:643='/\(/', <REGULAR_LITERAL>, <14>, start: <line: 30, position: 10>, stop: <line: 30, position: 14>]
   * [@116, 643:644=';', <SEMI>, <5>, start: <line: 30, position: 14>, stop: <line: 30, position: 15>]
   * [@117, 647:653='RPAREN', <TOKEN_REF>, <3>, start: <line: 31, position: 2>, stop: <line: 31, position: 8>]
   * [@118, 653:654=':', <COLON>, <4>, start: <line: 31, position: 8>, stop: <line: 31, position: 9>]
   * [@119, 655:659='/\)/', <REGULAR_LITERAL>, <14>, start: <line: 31, position: 10>, stop: <line: 31, position: 14>]
   * [@120, 659:660=';', <SEMI>, <5>, start: <line: 31, position: 14>, stop: <line: 31, position: 15>]
   * [@121, 663:677='STRING_LITERAL', <TOKEN_REF>, <3>, start: <line: 32, position: 2>, stop: <line: 32, position: 16>]
   * [@122, 677:678=':', <COLON>, <4>, start: <line: 32, position: 16>, stop: <line: 32, position: 17>]
   * [@123, 679:772='/"((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*"/', <REGULAR_LITERAL>, <14>, start: <line: 32, position: 18>, stop: <line: 32, position: 111>]
   * [@124, 772:773=';', <SEMI>, <5>, start: <line: 32, position: 111>, stop: <line: 32, position: 112>]
   * [@125, 776:791='REGULAR_LITERAL', <TOKEN_REF>, <3>, start: <line: 33, position: 2>, stop: <line: 33, position: 17>]
   * [@126, 791:792=':', <COLON>, <4>, start: <line: 33, position: 17>, stop: <line: 33, position: 18>]
   * [@127, 793:812='/\/(\\\/|[^\/])+\//', <REGULAR_LITERAL>, <14>, start: <line: 33, position: 19>, stop: <line: 33, position: 38>]
   * [@128, 812:813=';', <SEMI>, <5>, start: <line: 33, position: 38>, stop: <line: 33, position: 39>]
   * [@129, 816:827='WHITE_SPACE', <TOKEN_REF>, <3>, start: <line: 34, position: 2>, stop: <line: 34, position: 13>]
   * [@130, 827:828=':', <COLON>, <4>, start: <line: 34, position: 13>, stop: <line: 34, position: 14>]
   * [@131, 829:843='/[ \r\n\t\f]+/', <REGULAR_LITERAL>, <14>, start: <line: 34, position: 15>, stop: <line: 34, position: 29>]
   * [@132, 843:844=';', <SEMI>, <5>, start: <line: 34, position: 29>, stop: <line: 34, position: 30>]
   * [@133, 850:850='_STOP', <_STOP>, <1>, start: <line: 36, position: 2>, stop: <line: 36, position: 2>]
   */

  /**
   * _START, RULE_REF, COLON, LPAREN, RULE_REF, OR, RULE_REF, 
   * RPAREN, STAR, SEMI, RULE_REF, COLON, TOKEN_REF, TOKEN_REF, 
   * RULE_REF, TOKEN_REF, SEMI, RULE_REF, COLON, RULE_REF, LPAREN, 
   * TOKEN_REF, RULE_REF, RPAREN, STAR, SEMI, RULE_REF, COLON, 
   * RULE_REF, RULE_REF, STAR, OR, RULE_REF, SEMI, RULE_REF, 
   * COLON, TOKEN_REF, SEMI, RULE_REF, COLON, LPAREN, TOKEN_REF, 
   * OR, TOKEN_REF, OR, TOKEN_REF, OR, TOKEN_REF, RULE_REF, 
   * TOKEN_REF, RPAREN, RULE_REF, QUESTION, SEMI, RULE_REF, 
   * COLON, LPAREN, TOKEN_REF, OR, TOKEN_REF, OR, TOKEN_REF, 
   * RPAREN, TOKEN_REF, QUESTION, SEMI, RULE_REF, COLON, 
   * TOKEN_REF, TOKEN_REF, RULE_REF, TOKEN_REF, SEMI, RULE_REF, 
   * COLON, TOKEN_REF, SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, 
   * SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, SEMI, TOKEN_REF, 
   * COLON, REGULAR_LITERAL, SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, 
   * SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, SEMI, TOKEN_REF, 
   * COLON, REGULAR_LITERAL, SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, 
   * SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, SEMI, TOKEN_REF, COLON, 
   * REGULAR_LITERAL, SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, 
   * SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, SEMI, TOKEN_REF, COLON, 
   * REGULAR_LITERAL, SEMI, TOKEN_REF, COLON, REGULAR_LITERAL, SEMI, 
   * TOKEN_REF, COLON, REGULAR_LITERAL, SEMI, _STOP,
   *  */  



  let lexer = ChiruLexer::new(input);


  for token in lexer.iter() {
    println!("{}", token);
  }

  let mut stream = TokenStream::new(&lexer, 0);

  assert_eq!(stream.next().unwrap().token_name, "_START");
  assert_eq!(stream.next().unwrap().token_name, "RULE_REF");
  assert_eq!(stream.next().unwrap().token_name, "COLON");
  assert_eq!(stream.look_ahead(1).unwrap().token_name, "LPAREN");
  assert_eq!(stream.look_back(1).unwrap().token_name, "COLON");
  assert_eq!(stream.look_ahead(9).unwrap().token_name, "COLON");
  assert_eq!(stream.look_ahead(14).unwrap().token_name, "SEMI");


  stream.consume().unwrap();
  stream.consume().unwrap();
  stream.consume().unwrap();
  assert_eq!(stream.look_back(1).unwrap().token_name, "OR");
  assert_eq!(stream.look_back(3).unwrap().token_name, "LPAREN");
  assert_eq!(stream.release().unwrap().token_name, "OR");

}


