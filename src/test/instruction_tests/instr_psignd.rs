use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 221], OperandSize::Dword)
}

#[test]
fn psignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(ESI, 1465547814, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 158, 38, 124, 90, 87], OperandSize::Dword)
}

#[test]
fn psignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 211], OperandSize::Qword)
}

#[test]
fn psignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 14], OperandSize::Qword)
}

#[test]
fn psignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 229], OperandSize::Dword)
}

#[test]
fn psignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 2061856794, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 164, 75, 26, 112, 229, 122], OperandSize::Dword)
}

#[test]
fn psignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 212], OperandSize::Qword)
}

#[test]
fn psignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 911842858, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 184, 42, 158, 89, 54], OperandSize::Qword)
}

