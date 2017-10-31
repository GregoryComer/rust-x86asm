use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 201], OperandSize::Dword)
}

#[test]
fn paddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 54], OperandSize::Dword)
}

#[test]
fn paddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 209], OperandSize::Qword)
}

#[test]
fn paddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 57], OperandSize::Qword)
}

#[test]
fn paddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 231], OperandSize::Dword)
}

#[test]
fn paddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1181615935, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 140, 184, 63, 7, 110, 70], OperandSize::Dword)
}

#[test]
fn paddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 214], OperandSize::Qword)
}

#[test]
fn paddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 36, 186], OperandSize::Qword)
}

