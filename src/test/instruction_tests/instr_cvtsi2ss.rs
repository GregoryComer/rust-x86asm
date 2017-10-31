use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 247], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1324094475, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 60, 157, 11, 20, 236, 78], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 221], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 436569985, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 140, 122, 129, 135, 5, 26], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 252], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDX, 1503658098, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 162, 114, 0, 160, 89], OperandSize::Qword)
}

