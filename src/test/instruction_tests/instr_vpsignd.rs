use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 10, 222], OperandSize::Dword)
}

#[test]
fn vpsignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 10, 20, 183], OperandSize::Dword)
}

#[test]
fn vpsignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 10, 213], OperandSize::Qword)
}

#[test]
fn vpsignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RAX, 1893392722, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 10, 176, 82, 225, 218, 112], OperandSize::Qword)
}

#[test]
fn vpsignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 10, 211], OperandSize::Dword)
}

#[test]
fn vpsignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 10, 36, 216], OperandSize::Dword)
}

#[test]
fn vpsignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 10, 249], OperandSize::Qword)
}

#[test]
fn vpsignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1753249789, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 10, 28, 117, 253, 119, 128, 104], OperandSize::Qword)
}

