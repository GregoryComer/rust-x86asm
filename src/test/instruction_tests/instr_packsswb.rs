use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packsswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 231], OperandSize::Dword)
}

#[test]
fn packsswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EDI, 1219984803, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 159, 163, 125, 183, 72], OperandSize::Dword)
}

#[test]
fn packsswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 211], OperandSize::Qword)
}

#[test]
fn packsswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1628899094, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 12, 205, 22, 7, 23, 97], OperandSize::Qword)
}

#[test]
fn packsswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 231], OperandSize::Dword)
}

#[test]
fn packsswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 49], OperandSize::Dword)
}

#[test]
fn packsswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 203], OperandSize::Qword)
}

#[test]
fn packsswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RBX, 795889888, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 147, 224, 80, 112, 47], OperandSize::Qword)
}

