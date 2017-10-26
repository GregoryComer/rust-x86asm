use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 234], OperandSize::Dword)
}

#[test]
fn andnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 542365423, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 156, 186, 239, 214, 83, 32], OperandSize::Dword)
}

#[test]
fn andnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 196], OperandSize::Qword)
}

#[test]
fn andnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 85, 63], OperandSize::Qword)
}

