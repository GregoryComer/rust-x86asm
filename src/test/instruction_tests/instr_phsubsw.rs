use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 251], OperandSize::Dword)
}

#[test]
fn phsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 2092314247, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 132, 254, 135, 46, 182, 124], OperandSize::Dword)
}

#[test]
fn phsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 235], OperandSize::Qword)
}

#[test]
fn phsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(RCX, 1123127408, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 153, 112, 144, 241, 66], OperandSize::Qword)
}

#[test]
fn phsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 226], OperandSize::Dword)
}

#[test]
fn phsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 8], OperandSize::Dword)
}

#[test]
fn phsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 234], OperandSize::Qword)
}

#[test]
fn phsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 690265437, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 36, 133, 93, 157, 36, 41], OperandSize::Qword)
}

