use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 204], OperandSize::Dword)
}

#[test]
fn andnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1935124344, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 12, 93, 120, 167, 87, 115], OperandSize::Dword)
}

#[test]
fn andnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 197], OperandSize::Qword)
}

#[test]
fn andnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 12806550, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 188, 191, 150, 105, 195, 0], OperandSize::Qword)
}

