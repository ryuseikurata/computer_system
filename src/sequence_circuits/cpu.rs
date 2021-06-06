use crate::{
    adders::alu,
    gates::{and, mux, mux16, not, or},
};

use super::{counter_circuit::Counter, register::Register, word::Word};

pub struct CPU {
    a_register: Register,
    d_register: Register,
    out_memory: Word,
    write_to_memory: bool,
    address_memory: [bool; 15],
    pc: Counter,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: Register::new(),
            d_register: Register::new(),
            out_memory: [false; 16],
            write_to_memory: false,
            address_memory: [false; 15],
            pc: Counter::new(),
        }
    }

    pub fn cycle(&mut self, in_memory: Word, instruction: Word, reset: bool) {
        let is_c_command = instruction[0]; // A命令かC命令か
                                           // C命令の動作設定
        let a = instruction[3];
        // Comp領域
        let [c1, c2, c3, c4, c5, c6] = [
            instruction[4],
            instruction[5],
            instruction[6],
            instruction[7],
            instruction[8],
            instruction[9],
        ];
        // comp領域を格納する場所を決定
        // d1 = true => Aレジスタに格納
        // d2 = true => Dレジスタに格納
        // d3 = true => Memoryに格納
        let [d1, d2, d3] = [instruction[10], instruction[11], instruction[12]];

        /*
        ALUの計算
        */
        // Memoryを使うかどうか
        let is_use_memory = and::calc(is_c_command, a);
        // 入力xをzeroにするか
        let zx = and::calc(is_c_command, c1);
        // 入力xを反転するか
        let nx = and::calc(is_c_command, c2);
        // 入力yをzeroにするか
        let zy = and::calc(is_c_command, c3);
        // 入力yを反転するか
        let ny = and::calc(is_c_command, c4);
        // trueは加算、falseはand演算
        let f = and::calc(is_c_command, c5);
        // 出力を反転するか
        let no = and::calc(is_c_command, c6);
        let x = self.d_register.out();
        // Memoryを使用、もしくはAレジスタを使用
        let y = mux16::calc(self.a_register.out(), in_memory, is_use_memory);
        let (alu_out_result, alu_out_is_zero, alu_out_is_negate) =
            alu::calc(x, y, zx, nx, zy, ny, f, no);
        self.out_memory = alu_out_result;

        /*
        Aレジスタへの書き込み

        - Aレジスタへの入力を決定
        */
        // A命令の場合もしくはC命令のdistが1の場合、Aレジスタに書き込むかどうかを判断
        let is_load_a = or::calc(!is_c_command, d1);
        // A命令であればinstruction, C命令であればresultをAに記録
        let to_a = mux16::calc(instruction, alu_out_result, is_c_command);
        self.a_register.clock(to_a, is_load_a);
        let out_a = self.a_register.out();
        // a[0]はA命令か,C命令かの判断に使われるだけなので切り捨てることができる
        self.address_memory = [
            out_a[1], out_a[2], out_a[3], out_a[4], out_a[5], out_a[6], out_a[7], out_a[8],
            out_a[9], out_a[10], out_a[11], out_a[12], out_a[13], out_a[14], out_a[15],
        ];

        /*
        Dレジスタへの書き込み
        C命令かつdestのd2(Dレジスタに計算結果を格納する）がONであればDレジスタにロードする
        */
        let is_load_d = and::calc(is_c_command, d2);
        self.d_register.clock(alu_out_result, is_load_d);

        /*
        WriteMemoryへの書き込み
        */
        self.write_to_memory = and::calc(is_c_command, d3);

        /*
        GT(GreaterThan)判定
        */
        let is_positive = not::calc(alu_out_is_negate);
        let is_not_zero = not::calc(alu_out_is_zero);
        let is_gt = and::calc(is_positive, is_not_zero);

        /*
        Jump判定(GT)
        - C命令かつj領域のj3がtrueかどうかで判定
        */
        let is_jump_gt = and::calc(is_c_command, instruction[15]);

        /*
        Jump(GT)によるPCの書き換え判定
        - GTかつJumpする場合に書き換える
        */
        let is_pc_load_jump_gt = and::calc(is_jump_gt, is_gt);

        /*
         EQ(equal)判定
         なぜZeroでEqual判定できるかは謎
        */
        let is_equal = alu_out_is_zero;
        /*
        Jump(EQ)判定
        j2がtrueかどうかで判断
        */
        let is_jump_equql = and::calc(is_c_command, instruction[14]);
        /*
        Jump(EQ)によるPCの書き換え判定
        */
        let is_pc_load_jump_equal = and::calc(is_equal, is_jump_equql);

        /*
         LT(less than)判定
        */
        let is_lt = alu_out_is_negate;
        /*
        Jump(LT)判定
        j2がtrueかどうかで判断
        */
        let is_jump_lt = and::calc(is_c_command, instruction[13]);
        /*
        Jump(LT)によるPCの書き換え判定
        */
        let is_pc_load_jump_lt = and::calc(is_lt, is_jump_lt);

        /*
        Jump(GE greater equal)によるPCの書き換え判定
        */
        let is_pc_load_jump_ge = or::calc(is_pc_load_jump_gt, is_pc_load_jump_equal);

        /*
        PCを書き換えるかの最終判定
        */
        let is_pc_load = or::calc(is_pc_load_jump_lt, is_pc_load_jump_ge);

        /*
        PC書き換え
        - PcLoadがtrueの場合はAレジスタの値で書き換えをする
        - そうでなければインクリメント
        - なぜincはtrueにしていいのかは謎
        */
        self.pc.calc(out_a, true, is_pc_load, reset);
    }
    /**
    出力 (i xx a cccccc ddd jjj) \
    i: 命令の種類 false=>A命令, true=>C命令 \
    C命令の場合、以下のように読み取られる。A命令の場合、定数値として解釈される。\
    a,c => comp領域 \
    d => dest領域 \
    j => jump領域
    */
    pub fn decode(instruction: Word) -> (bool, [bool; 2], bool, [bool; 6], [bool; 3], [bool; 3]) {
        (
            instruction[0],
            [instruction[1], instruction[2]],
            instruction[3],
            [
                instruction[4],
                instruction[5],
                instruction[6],
                instruction[7],
                instruction[8],
                instruction[9],
            ],
            [instruction[10], instruction[11], instruction[12]],
            [instruction[13], instruction[14], instruction[15]],
        )
    }

    pub fn out(&self) -> (Word, bool, [bool; 15], Counter) {
        (
            self.out_memory,
            self.write_to_memory,
            self.address_memory,
            self.pc,
        )
    }

    pub fn pc(&self) -> &Counter {
        &self.pc
    }

    pub fn d(&self) -> Word {
        self.a_register.out()
    }

    pub fn a(&self) -> Word {
        self.d_register.out()
    }
}

#[cfg(test)]
mod tests {
    use crate::{const_values, sequence_circuits::word::Word};

    use super::CPU;

    // 1 + 1を実装
    // Memoryに1を設定
    // C命令で1をプラスするという演算を仕込む
    /// no jump 000
    // 数値参照のためにはシンボル実装しなければいけない
    //
    //  C命令+1 => [true, true, true, a, true, true, true, true, true, true, d1, d2, d3, j1, j2, j3 ]
    // 1つ目はtrue、2,3はtrueにする必要がある c領域は、A+1(アドレスレジスタにあるものから+1)する
    //
    #[test]
    fn increment() {
        let in_memory = const_values::ONE;
        let instruction = c_instruction();
        let mut cpu = CPU::new();
        cpu.cycle(in_memory, instruction, false);
        let out = const_values::TWO;
        assert_eq!(out, cpu.out_memory);
    }

    fn c_instruction() -> Word {
        // Aレジスタ、もしくはAレジスタとDレジスタによる処理はfalse
        let a = true;
        // Compニーモニック(M+1)
        let [c1, c2, c3, c4, c5, c6] = [true, true, false, true, true, true];
        // d1: Aレジスタに格納するか
        // d2: Dレジスタに格納するか
        // d3: Memoryに格納するか
        // Memoryに保存したいのでd3のみtrue
        let [d1, d2, d3] = [false, false, true];
        let [j1, j2, j3] = [false, false, false];
        [
            true, true, true, a, c1, c2, c3, c4, c5, c6, d1, d2, d3, j1, j2, j3,
        ]
    }
}
