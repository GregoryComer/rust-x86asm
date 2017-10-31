use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 255], OperandSize::Dword)
}

#[test]
fn andpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 641988297, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 60, 181, 201, 246, 67, 38], OperandSize::Dword)
}

#[test]
fn andpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 246], OperandSize::Qword)
}

#[test]
fn andpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RCX, 1892194398, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 161, 94, 152, 200, 112], OperandSize::Qword)
}

