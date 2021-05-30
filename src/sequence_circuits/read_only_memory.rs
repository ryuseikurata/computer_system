use std::fs::File;

use super::random_access_memory::RAM4K;

/// 機械語で書かれたプログラムの命令を一つずつ、アドレスの0番目から順に保持する構造体
pub struct ROM32K {
  rams: [RAM4K; 8]
}

impl ROM32K {
  pub fn load(&mut self, file_name: &str) {
    let file = File::open(file_name.clone()).expect(&format!("Fail to Open {}", file_name));
    
  }
}
