use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 253], OperandSize::Dword)
}

#[test]
fn xorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 20, 71], OperandSize::Dword)
}

#[test]
fn xorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 244], OperandSize::Qword)
}

#[test]
fn xorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 695030554, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 146, 26, 83, 109, 41], OperandSize::Qword)
}

