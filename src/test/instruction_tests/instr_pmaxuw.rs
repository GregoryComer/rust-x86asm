use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 205], OperandSize::Dword)
}

#[test]
fn pmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 11], OperandSize::Dword)
}

#[test]
fn pmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 210], OperandSize::Qword)
}

#[test]
fn pmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1924711317, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 44, 141, 149, 195, 184, 114], OperandSize::Qword)
}

