

pub trait ErrorListener {
  fn syntax_error(&self) {}
}


// 定义一些需要的 error_listener
pub struct BaseErrorListener;
impl ErrorListener for BaseErrorListener {
}


pub struct ConsoleErrorListener;
impl ErrorListener for ConsoleErrorListener {
}

pub struct DiagnosticErrorListener;
impl ErrorListener for DiagnosticErrorListener {
}

pub struct ProxyErrorListener ;
impl ErrorListener for ProxyErrorListener {
}

