use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 216], OperandSize::Dword)
}

#[test]
fn subpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 12, 64], OperandSize::Dword)
}

#[test]
fn subpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 236], OperandSize::Qword)
}

#[test]
fn subpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1053021352, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 148, 122, 168, 212, 195, 62], OperandSize::Qword)
}

