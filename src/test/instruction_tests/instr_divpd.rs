use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 192], OperandSize::Dword)
}

#[test]
fn divpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 44, 118], OperandSize::Dword)
}

#[test]
fn divpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 194], OperandSize::Qword)
}

#[test]
fn divpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDX, 819642365, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 94, 186, 253, 191, 218, 48], OperandSize::Qword)
}

