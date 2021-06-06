use computer_components::{computer::Computer, keyboard::Keyboard, screen::Screen};
use sequence_circuits::read_only_memory::ROM32K;
mod adders;
mod computer_components;
mod const_values;
mod gates;
mod sequence_circuits;

fn main() {
    let mut computer = Computer::<dyn Screen, dyn Keyboard>::new();
    // 本当であれば、Fileから取り出す
    let test_rom = ROM32K::new();
    computer.set_rom(test_rom);
    // ROMにsetされたバイナリコードが、clockのタイミングによって実行される。Screen情報なども入っている
    computer.clock(true);

    // KeyboardEventにしたがって、Keyboardをset
    computer.set_keyboard_state();

    // 一定時間ごとにfalse。つまりtokする
    computer.clock(false);
}
