use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(XMM5)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 40, 239], OperandSize::Dword)
}

#[test]
fn vpmovm2b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(XMM16)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 40, 193], OperandSize::Qword)
}

#[test]
fn vpmovm2b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(YMM7)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 40, 252], OperandSize::Dword)
}

#[test]
fn vpmovm2b_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(YMM1)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 40, 206], OperandSize::Qword)
}

#[test]
fn vpmovm2b_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 40, 251], OperandSize::Dword)
}

#[test]
fn vpmovm2b_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 40, 223], OperandSize::Qword)
}

