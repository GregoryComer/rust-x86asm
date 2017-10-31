use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 156, 204, 217], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 204, 4, 194], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 204, 50], OperandSize::Dword)
}

#[test]
fn vrsqrt28ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 125, 157, 204, 219], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 702246834, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 205, 204, 188, 240, 178, 111, 219, 41], OperandSize::Qword)
}

#[test]
fn vrsqrt28ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28PS, operand1: Some(Direct(ZMM31)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 222, 204, 56], OperandSize::Qword)
}

