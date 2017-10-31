use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM5)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 56, 235], OperandSize::Dword)
}

#[test]
fn vpmovm2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 56, 199], OperandSize::Qword)
}

#[test]
fn vpmovm2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM2)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 56, 211], OperandSize::Dword)
}

#[test]
fn vpmovm2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM28)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 56, 231], OperandSize::Qword)
}

#[test]
fn vpmovm2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 56, 211], OperandSize::Dword)
}

#[test]
fn vpmovm2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 56, 205], OperandSize::Qword)
}

