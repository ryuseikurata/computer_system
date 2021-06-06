use super::{keyboard::Keyboard, screen::Screen};
use crate::{
    gates::{and, dmux4way, not, or},
    sequence_circuits::{random_access_memory::RAM16K, word::Word},
};

pub struct Memory<S: Screen, K: Keyboard> {
    address: [bool; 15],
    ram: RAM16K,
    screen: S,
    keyboard: K,
}

impl<S: Screen, K: Keyboard> Memory<S, K> {
    pub fn new() -> Self {
        Self {
            address: [false; 15],
            ram: RAM16K::new(),
            screen: Screen::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn clock(&mut self, address: &[bool; 15], load: bool, input: Word) {
        // ポインタの移動を行わず(参照外し)、値だけ取得
        self.address = *address;

        // アドレス13,14で判断
        // address[14] = true かつ address[13] = trueはキーボード  => [false, false, false, input]
        // address[14] = true かつ address[13] = falseはスクリーン => [false, input, false, false]
        // それ以外はRAM
        let [is_load_ram_1, is_load_screen, is_load_ram_2, is_load_key_board] =
            dmux4way::calc(load, [address[14], address[13]]);
        let is_load_ram = or::calc(is_load_ram_1, is_load_ram_2);

        // screen書き込み
        // keyboard書き込み
        // ram書き込み
        self.ram.clock(
            [
                address[0],
                address[1],
                address[2],
                address[3],
                address[4],
                address[5],
                address[6],
                address[7],
                address[8],
                address[9],
                address[10],
                address[11],
                address[12],
                address[13],
            ],
            input,
            is_load_ram,
        );
    }
}
