use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn orpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 215], OperandSize::Dword)
}

#[test]
fn orpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 40], OperandSize::Dword)
}

#[test]
fn orpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 238], OperandSize::Qword)
}

#[test]
fn orpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 326016394, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 148, 142, 138, 157, 110, 19], OperandSize::Qword)
}

