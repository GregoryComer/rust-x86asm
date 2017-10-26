use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vtestps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 197], OperandSize::Dword)
}

#[test]
fn vtestps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 20, 129], OperandSize::Dword)
}

#[test]
fn vtestps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 200], OperandSize::Qword)
}

#[test]
fn vtestps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 695358993, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 14, 36, 133, 17, 86, 114, 41], OperandSize::Qword)
}

#[test]
fn vtestps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 192], OperandSize::Dword)
}

#[test]
fn vtestps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 40], OperandSize::Dword)
}

#[test]
fn vtestps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 220], OperandSize::Qword)
}

#[test]
fn vtestps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VTESTPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 388574819, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 14, 132, 75, 99, 46, 41, 23], OperandSize::Qword)
}

