use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn hsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 253], OperandSize::Dword)
}

#[test]
fn hsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 36, 143], OperandSize::Dword)
}

#[test]
fn hsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 193], OperandSize::Qword)
}

#[test]
fn hsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1219720293, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 4, 125, 101, 116, 179, 72], OperandSize::Qword)
}

