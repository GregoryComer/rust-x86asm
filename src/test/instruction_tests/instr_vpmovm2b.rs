use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2b_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(XMM6)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 40, 243], OperandSize::Dword)
}

#[test]
fn vpmovm2b_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(XMM28)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 40, 228], OperandSize::Qword)
}

#[test]
fn vpmovm2b_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(YMM7)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 40, 250], OperandSize::Dword)
}

#[test]
fn vpmovm2b_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(YMM2)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 40, 215], OperandSize::Qword)
}

#[test]
fn vpmovm2b_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 40, 237], OperandSize::Dword)
}

#[test]
fn vpmovm2b_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2B, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 40, 222], OperandSize::Qword)
}

