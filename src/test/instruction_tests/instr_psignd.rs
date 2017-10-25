use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 195], OperandSize::Dword)
}

#[test]
fn psignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDX, 1309470247, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 186, 39, 238, 12, 78], OperandSize::Dword)
}

#[test]
fn psignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 195], OperandSize::Qword)
}

#[test]
fn psignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1827312203, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 60, 205, 75, 146, 234, 108], OperandSize::Qword)
}

#[test]
fn psignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 218], OperandSize::Dword)
}

#[test]
fn psignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 482069400, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 156, 211, 152, 203, 187, 28], OperandSize::Dword)
}

#[test]
fn psignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 232], OperandSize::Qword)
}

#[test]
fn psignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 2088415970, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 148, 208, 226, 178, 122, 124], OperandSize::Qword)
}

