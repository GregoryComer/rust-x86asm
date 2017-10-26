use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn comisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 252], OperandSize::Dword)
}

#[test]
fn comisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EBX, 930124558, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 147, 14, 147, 112, 55], OperandSize::Dword)
}

#[test]
fn comisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 250], OperandSize::Qword)
}

#[test]
fn comisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::COMISD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 440126210, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 47, 180, 80, 2, 203, 59, 26], OperandSize::Qword)
}

