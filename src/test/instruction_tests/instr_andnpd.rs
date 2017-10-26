use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 254], OperandSize::Dword)
}

#[test]
fn andnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1250257668, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 188, 127, 4, 107, 133, 74], OperandSize::Dword)
}

#[test]
fn andnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 253], OperandSize::Qword)
}

#[test]
fn andnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 24], OperandSize::Qword)
}

