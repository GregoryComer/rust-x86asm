use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 40, 226], OperandSize::Dword)
}

#[test]
fn vpmovm2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM20)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 254, 8, 40, 229], OperandSize::Qword)
}

#[test]
fn vpmovm2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM0)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 40, 194], OperandSize::Dword)
}

#[test]
fn vpmovm2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM25)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 254, 40, 40, 204], OperandSize::Qword)
}

#[test]
fn vpmovm2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 40, 201], OperandSize::Dword)
}

#[test]
fn vpmovm2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 254, 72, 40, 209], OperandSize::Qword)
}

