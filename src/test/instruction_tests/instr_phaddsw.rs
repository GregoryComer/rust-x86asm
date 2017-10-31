use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 243], OperandSize::Dword)
}

#[test]
fn phaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1284725433, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 52, 125, 185, 90, 147, 76], OperandSize::Dword)
}

#[test]
fn phaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 200], OperandSize::Qword)
}

#[test]
fn phaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 36, 202], OperandSize::Qword)
}

#[test]
fn phaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 208], OperandSize::Dword)
}

#[test]
fn phaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 52, 223], OperandSize::Dword)
}

#[test]
fn phaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 222], OperandSize::Qword)
}

#[test]
fn phaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 11], OperandSize::Qword)
}

