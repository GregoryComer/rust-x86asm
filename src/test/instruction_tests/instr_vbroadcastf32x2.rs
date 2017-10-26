use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 25, 197], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ESI, 749618913, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 25, 158, 225, 70, 174, 44], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM13)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 173, 25, 239], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 884510582, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 25, 4, 141, 118, 143, 184, 52], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 25, 226], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(ECX, 1664620330, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 25, 129, 42, 23, 56, 99], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 125, 205, 25, 212], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 25, 30], OperandSize::Qword)
}

