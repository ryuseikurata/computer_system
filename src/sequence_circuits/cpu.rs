use crate::{
    adders::alu,
    gates::{and, mux16, not, or},
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
        let is_c_command = instruction[15]; // A命令かC命令か

        /*
        ALUの計算
        */
        let use_m = and::calc(is_c_command, instruction[3]);
        let zero_d = and::calc(is_c_command, instruction[4]);
        let negate_d = and::calc(is_c_command, instruction[5]);
        let zero_a = and::calc(is_c_command, instruction[6]);
        let negate_a = and::calc(is_c_command, instruction[7]);
        let f = and::calc(is_c_command, instruction[8]);
        let negate_alu = and::calc(is_c_command, instruction[9]);
        let d = self.d_register.out();
        let a = mux16::calc(self.a_register.out(), in_memory, use_m);
        let (alu_out_result, alu_out_is_zero, alu_out_is_negate) =
            alu::calc(d, a, zero_d, negate_d, zero_a, negate_a, f, negate_alu);
        self.out_memory = alu_out_result;

        /*
        Aレジスタへの書き込み

        - Aレジスタへの入力を決定
        - A命令であればinstruction, C命令であればresultをAに記録
        - A命令の場合もしくはC命令のdistが1の場合、Aレジスタに書き込む
        */
        let is_load_a = or::calc(!is_c_command, instruction[10]);
        self.a_register.clock(
            mux16::calc(instruction, alu_out_result, is_c_command),
            is_load_a,
        );
        let a = self.a_register.out();
        self.address_memory = [
            a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
            a[14], a[15],
        ];

        /*
        Dレジスタへの書き込み
        C命令かつdestのd2(Dレジスタに計算結果を格納する）がONであればDレジスタにロードする
        */
        let is_load_d = and::calc(is_c_command, instruction[11]);
        self.d_register.clock(alu_out_result, is_load_d);

        /*
        WriteMemoryへの書き込み
        */
        self.write_to_memory = and::calc(is_c_command, instruction[12]);

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
        self.pc.calc(a, true, is_pc_load, reset);
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
}
