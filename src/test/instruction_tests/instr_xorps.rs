use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 243], OperandSize::Dword)
}

#[test]
fn xorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ESI, 405950157, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 166, 205, 78, 50, 24], OperandSize::Dword)
}

#[test]
fn xorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 213], OperandSize::Qword)
}

#[test]
fn xorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RCX, 453983912, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 153, 168, 62, 15, 27], OperandSize::Qword)
}

