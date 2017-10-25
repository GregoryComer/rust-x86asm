use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 67, 216, 100], OperandSize::Dword)
}

#[test]
fn vshufi32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 109, 206, 67, 28, 71, 126], OperandSize::Dword)
}

#[test]
fn vshufi32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EBX, 1425174652, Some(OperandSize::Dword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 117, 221, 67, 187, 124, 112, 242, 84, 78], OperandSize::Dword)
}

#[test]
fn vshufi32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM19)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 51, 37, 203, 67, 195, 100], OperandSize::Qword)
}

#[test]
fn vshufi32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 125, 201, 67, 60, 78, 37], OperandSize::Qword)
}

#[test]
fn vshufi32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 117, 210, 67, 1, 34], OperandSize::Qword)
}

