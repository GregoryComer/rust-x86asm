use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 2, 235], OperandSize::Dword)
}

#[test]
fn vphaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 365521277, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 2, 12, 197, 125, 105, 201, 21], OperandSize::Dword)
}

#[test]
fn vphaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 2, 219], OperandSize::Qword)
}

#[test]
fn vphaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 2, 4, 153], OperandSize::Qword)
}

#[test]
fn vphaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 2, 242], OperandSize::Dword)
}

#[test]
fn vphaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1995173071, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 2, 148, 248, 207, 236, 235, 118], OperandSize::Dword)
}

#[test]
fn vphaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 2, 236], OperandSize::Qword)
}

#[test]
fn vphaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 2, 28, 126], OperandSize::Qword)
}

