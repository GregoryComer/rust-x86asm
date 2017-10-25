use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 56, 231], OperandSize::Dword)
}

#[test]
fn vpmovm2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM7)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 56, 253], OperandSize::Qword)
}

#[test]
fn vpmovm2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM0)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 56, 194], OperandSize::Dword)
}

#[test]
fn vpmovm2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM23)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 56, 249], OperandSize::Qword)
}

#[test]
fn vpmovm2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 56, 198], OperandSize::Dword)
}

#[test]
fn vpmovm2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 56, 209], OperandSize::Qword)
}

