use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 248], OperandSize::Dword)
}

#[test]
fn subss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1347351888, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 20, 181, 80, 245, 78, 80], OperandSize::Dword)
}

#[test]
fn subss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 192], OperandSize::Qword)
}

#[test]
fn subss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 4, 64], OperandSize::Qword)
}

