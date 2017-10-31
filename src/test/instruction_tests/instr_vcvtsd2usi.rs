use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 24, 121, 250], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(EDX, 1348679703, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 138, 23, 56, 99, 80], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 127, 88, 121, 205], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 120458089, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 36, 189, 105, 11, 46, 7], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 255, 56, 121, 251], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 620936004, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 121, 188, 123, 68, 187, 2, 37], OperandSize::Qword)
}

