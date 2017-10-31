use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 2, 216], OperandSize::Dword)
}

#[test]
fn vphaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 2, 10], OperandSize::Dword)
}

#[test]
fn vphaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 2, 249], OperandSize::Qword)
}

#[test]
fn vphaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 804295495, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 2, 132, 72, 71, 147, 240, 47], OperandSize::Qword)
}

#[test]
fn vphaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 2, 213], OperandSize::Dword)
}

#[test]
fn vphaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1209017142, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 2, 20, 181, 54, 35, 16, 72], OperandSize::Dword)
}

#[test]
fn vphaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 2, 201], OperandSize::Qword)
}

#[test]
fn vphaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RCX, 1754296497, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 2, 137, 177, 112, 144, 104], OperandSize::Qword)
}

