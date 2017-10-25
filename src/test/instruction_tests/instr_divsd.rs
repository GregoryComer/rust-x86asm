use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 214], OperandSize::Dword)
}

#[test]
fn divsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EAX, 1878776702, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 152, 126, 219, 251, 111], OperandSize::Dword)
}

#[test]
fn divsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 253], OperandSize::Qword)
}

#[test]
fn divsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 2001090438, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 140, 150, 134, 55, 70, 119], OperandSize::Qword)
}

