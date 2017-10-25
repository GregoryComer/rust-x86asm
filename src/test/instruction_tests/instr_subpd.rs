use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 235], OperandSize::Dword)
}

#[test]
fn subpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1717516191, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 20, 189, 159, 55, 95, 102], OperandSize::Dword)
}

#[test]
fn subpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 213], OperandSize::Qword)
}

#[test]
fn subpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 44, 184], OperandSize::Qword)
}

