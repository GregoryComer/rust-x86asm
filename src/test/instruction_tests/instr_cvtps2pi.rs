use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtps2pi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 215], OperandSize::Word)
}

#[test]
fn cvtps2pi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(BX, 226, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 175, 226, 0], OperandSize::Word)
}

#[test]
fn cvtps2pi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 226], OperandSize::Dword)
}

#[test]
fn cvtps2pi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 20, 139], OperandSize::Dword)
}

#[test]
fn cvtps2pi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 195], OperandSize::Qword)
}

#[test]
fn cvtps2pi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPS2PI, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1769949228, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 45, 172, 74, 44, 72, 127, 105], OperandSize::Qword)
}

