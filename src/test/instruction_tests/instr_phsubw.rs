use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 246], OperandSize::Dword)
}

#[test]
fn phsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2019694303, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 60, 197, 223, 22, 98, 120], OperandSize::Dword)
}

#[test]
fn phsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 250], OperandSize::Qword)
}

#[test]
fn phsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RDI, 1016128515, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 135, 3, 228, 144, 60], OperandSize::Qword)
}

#[test]
fn phsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 192], OperandSize::Dword)
}

#[test]
fn phsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 2079993894, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 132, 249, 38, 48, 250, 123], OperandSize::Dword)
}

#[test]
fn phsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 225], OperandSize::Qword)
}

#[test]
fn phsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 866062822, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 132, 179, 230, 17, 159, 51], OperandSize::Qword)
}

