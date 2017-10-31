use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ret_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(15668)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 52, 61], OperandSize::Word)
}

#[test]
fn ret_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(11522)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 2, 45], OperandSize::Dword)
}

#[test]
fn ret_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(21934)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 174, 85], OperandSize::Qword)
}

#[test]
fn ret_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(12616)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 72, 49], OperandSize::Word)
}

#[test]
fn ret_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(21327)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 79, 83], OperandSize::Dword)
}

#[test]
fn ret_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RET, operand1: Some(Literal16(15354)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[194, 250, 59], OperandSize::Qword)
}

