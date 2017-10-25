use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 213], OperandSize::Dword)
}

#[test]
fn packuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EBX, 1047488670, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 147, 158, 104, 111, 62], OperandSize::Dword)
}

#[test]
fn packuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 235], OperandSize::Qword)
}

#[test]
fn packuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 588773395, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 60, 221, 19, 248, 23, 35], OperandSize::Qword)
}

#[test]
fn packuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 235], OperandSize::Dword)
}

#[test]
fn packuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDX, 809214023, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 138, 71, 160, 59, 48], OperandSize::Dword)
}

#[test]
fn packuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 242], OperandSize::Qword)
}

#[test]
fn packuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 289397382, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 36, 213, 134, 218, 63, 17], OperandSize::Qword)
}

