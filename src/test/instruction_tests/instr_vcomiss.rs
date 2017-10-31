use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 252], OperandSize::Dword)
}

#[test]
fn vcomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 289041093, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 36, 149, 197, 106, 58, 17], OperandSize::Dword)
}

#[test]
fn vcomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 206], OperandSize::Qword)
}

#[test]
fn vcomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 1774584272, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 137, 208, 1, 198, 105], OperandSize::Qword)
}

#[test]
fn vcomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 47, 238], OperandSize::Dword)
}

#[test]
fn vcomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 40], OperandSize::Dword)
}

#[test]
fn vcomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 124, 24, 47, 234], OperandSize::Qword)
}

#[test]
fn vcomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 47, 35], OperandSize::Qword)
}

