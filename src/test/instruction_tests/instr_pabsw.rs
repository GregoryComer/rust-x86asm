use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 209], OperandSize::Dword)
}

#[test]
fn pabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EDI, 1105659256, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 159, 120, 5, 231, 65], OperandSize::Dword)
}

#[test]
fn pabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 201], OperandSize::Qword)
}

#[test]
fn pabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1863073426, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 148, 90, 146, 62, 12, 111], OperandSize::Qword)
}

#[test]
fn pabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 247], OperandSize::Dword)
}

#[test]
fn pabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 12, 201], OperandSize::Dword)
}

#[test]
fn pabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 202], OperandSize::Qword)
}

#[test]
fn pabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 212849931, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 52, 149, 11, 213, 175, 12], OperandSize::Qword)
}

