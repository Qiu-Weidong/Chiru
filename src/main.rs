use std::fs::File;

use syntaxis::tool::{serde_ast, gui::ast_drawer::ASTDrawer};



/**
 * syntaxis <grammar-filename> [-o 输出目录] [-encoding 编码] [-listener] [-visitor] [-package 模块名称] [-language 目标语言]
 *
 * syntaxis <grammar-filename> [-tokens] [-tree] [-gui] [-encoding 编码]
 */

fn main() {
  let file = File::open("src/tool/syntaxis/syntaxis.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();
 
  ASTDrawer::new().draw(&ast, "syntaxis", "syntaxis.html");

}



