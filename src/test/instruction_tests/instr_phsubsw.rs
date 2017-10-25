use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 217], OperandSize::Dword)
}

#[test]
fn phsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1894434125, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 132, 144, 77, 197, 234, 112], OperandSize::Dword)
}

#[test]
fn phsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 228], OperandSize::Qword)
}

#[test]
fn phsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 783998979, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 180, 215, 3, 224, 186, 46], OperandSize::Qword)
}

#[test]
fn phsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 247], OperandSize::Dword)
}

#[test]
fn phsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 44, 151], OperandSize::Dword)
}

#[test]
fn phsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 236], OperandSize::Qword)
}

#[test]
fn phsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1590840024, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 4, 157, 216, 74, 210, 94], OperandSize::Qword)
}

