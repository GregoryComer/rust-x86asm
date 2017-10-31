use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 56, 229], OperandSize::Dword)
}

#[test]
fn vpmovm2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM18)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 56, 212], OperandSize::Qword)
}

#[test]
fn vpmovm2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM6)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 56, 243], OperandSize::Dword)
}

#[test]
fn vpmovm2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM31)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 56, 249], OperandSize::Qword)
}

#[test]
fn vpmovm2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 56, 238], OperandSize::Dword)
}

#[test]
fn vpmovm2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 56, 226], OperandSize::Qword)
}

