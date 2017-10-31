use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 217], OperandSize::Dword)
}

#[test]
fn mulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 40], OperandSize::Dword)
}

#[test]
fn mulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 211], OperandSize::Qword)
}

#[test]
fn mulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1187358527, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 188, 190, 63, 167, 197, 70], OperandSize::Qword)
}

