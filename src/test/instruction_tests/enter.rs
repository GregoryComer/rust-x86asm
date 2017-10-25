use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn enter_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(15061)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 213, 58, 22], OperandSize::Word)
}

fn enter_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(9501)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 29, 37, 60], OperandSize::Dword)
}

fn enter_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ENTER, operand1: Some(Literal16(17128)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[200, 232, 66, 34], OperandSize::Qword)
}

