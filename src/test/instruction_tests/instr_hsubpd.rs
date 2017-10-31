use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn hsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 255], OperandSize::Dword)
}

#[test]
fn hsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 16], OperandSize::Dword)
}

#[test]
fn hsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 228], OperandSize::Qword)
}

#[test]
fn hsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDX, 678519122, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 130, 82, 97, 113, 40], OperandSize::Qword)
}

