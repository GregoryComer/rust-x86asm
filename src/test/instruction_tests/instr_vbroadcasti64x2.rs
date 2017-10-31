use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ESI, 1394093039, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 90, 134, 239, 43, 24, 83], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 372810574, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 90, 60, 77, 78, 163, 56, 22], OperandSize::Qword)
}

#[test]
fn vbroadcasti64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 512056853, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 90, 172, 120, 21, 94, 133, 30], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 90, 12, 113], OperandSize::Qword)
}

