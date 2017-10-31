use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 219], OperandSize::Dword)
}

#[test]
fn unpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1052173489, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 140, 176, 177, 228, 182, 62], OperandSize::Dword)
}

#[test]
fn unpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 218], OperandSize::Qword)
}

#[test]
fn unpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 31], OperandSize::Qword)
}

