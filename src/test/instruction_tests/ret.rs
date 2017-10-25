use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ret_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(10094)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 110, 39], OperandSize::Word)
}

fn ret_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(18734)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 46, 73], OperandSize::Dword)
}

fn ret_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(7233)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 65, 28], OperandSize::Qword)
}

fn ret_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(14071)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 247, 54], OperandSize::Word)
}

fn ret_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(8269)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 77, 32], OperandSize::Dword)
}

fn ret_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(20287)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 63, 79], OperandSize::Qword)
}

