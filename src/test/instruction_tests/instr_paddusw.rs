use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 204], OperandSize::Dword)
}

#[test]
fn paddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1292899628, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 4, 69, 44, 21, 16, 77], OperandSize::Dword)
}

#[test]
fn paddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 252], OperandSize::Qword)
}

#[test]
fn paddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDX, 972174259, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 146, 179, 51, 242, 57], OperandSize::Qword)
}

#[test]
fn paddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 218], OperandSize::Dword)
}

#[test]
fn paddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 49], OperandSize::Dword)
}

#[test]
fn paddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 205], OperandSize::Qword)
}

#[test]
fn paddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 49], OperandSize::Qword)
}

