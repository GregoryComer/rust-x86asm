use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 199], OperandSize::Dword)
}

#[test]
fn andpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 25], OperandSize::Dword)
}

#[test]
fn andpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 237], OperandSize::Qword)
}

#[test]
fn andpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 148760399, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 164, 145, 79, 231, 221, 8], OperandSize::Qword)
}

