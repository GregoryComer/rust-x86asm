use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 226], OperandSize::Dword)
}

#[test]
fn pmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 246763981, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 180, 183, 205, 81, 181, 14], OperandSize::Dword)
}

#[test]
fn pmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 203], OperandSize::Qword)
}

#[test]
fn pmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 63], OperandSize::Qword)
}

