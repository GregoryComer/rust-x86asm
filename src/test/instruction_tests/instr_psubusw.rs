use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 244], OperandSize::Dword)
}

#[test]
fn psubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 1], OperandSize::Dword)
}

#[test]
fn psubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 238], OperandSize::Qword)
}

#[test]
fn psubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1598580627, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 156, 78, 147, 103, 72, 95], OperandSize::Qword)
}

#[test]
fn psubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 230], OperandSize::Dword)
}

#[test]
fn psubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 23], OperandSize::Dword)
}

#[test]
fn psubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 231], OperandSize::Qword)
}

#[test]
fn psubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 2119147466, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 140, 89, 202, 159, 79, 126], OperandSize::Qword)
}

