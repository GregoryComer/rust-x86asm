use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 199], OperandSize::Dword)
}

#[test]
fn phsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 642097381, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 36, 125, 229, 160, 69, 38], OperandSize::Dword)
}

#[test]
fn phsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 229], OperandSize::Qword)
}

#[test]
fn phsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 52, 248], OperandSize::Qword)
}

#[test]
fn phsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 224], OperandSize::Dword)
}

#[test]
fn phsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 23], OperandSize::Dword)
}

#[test]
fn phsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 222], OperandSize::Qword)
}

#[test]
fn phsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 1041344899, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 187, 131, 169, 17, 62], OperandSize::Qword)
}

