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
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 15], OperandSize::Dword)
}

#[test]
fn psignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 196], OperandSize::Qword)
}

#[test]
fn psignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 4, 87], OperandSize::Qword)
}

#[test]
fn psignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 223], OperandSize::Dword)
}

#[test]
fn psignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 170408516, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 36, 69, 68, 58, 40, 10], OperandSize::Dword)
}

#[test]
fn psignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 241], OperandSize::Qword)
}

#[test]
fn psignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 44, 73], OperandSize::Qword)
}

