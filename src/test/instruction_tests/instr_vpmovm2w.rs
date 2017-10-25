use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovm2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM6)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 40, 243], OperandSize::Dword)
}

#[test]
fn vpmovm2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 8, 40, 226], OperandSize::Qword)
}

#[test]
fn vpmovm2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM2)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 40, 40, 213], OperandSize::Dword)
}

#[test]
fn vpmovm2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(YMM11)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 254, 40, 40, 222], OperandSize::Qword)
}

#[test]
fn vpmovm2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 254, 72, 40, 252], OperandSize::Dword)
}

#[test]
fn vpmovm2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVM2W, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 254, 72, 40, 231], OperandSize::Qword)
}

