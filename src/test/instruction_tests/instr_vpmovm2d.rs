use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM3)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 56, 220], OperandSize::Dword)
}

#[test]
fn vpmovm2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(XMM30)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 56, 246], OperandSize::Qword)
}

#[test]
fn vpmovm2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM1)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 56, 206], OperandSize::Dword)
}

#[test]
fn vpmovm2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(YMM15)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 56, 254], OperandSize::Qword)
}

#[test]
fn vpmovm2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 56, 197], OperandSize::Dword)
}

#[test]
fn vpmovm2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2D, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 56, 204], OperandSize::Qword)
}

