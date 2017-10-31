use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 202, 35, 199, 51], OperandSize::Dword)
}

#[test]
fn vshuff64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 205, 35, 25, 93], OperandSize::Dword)
}

#[test]
fn vshuff64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ECX, 161270027, Some(OperandSize::Qword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 213, 223, 35, 161, 11, 201, 156, 9, 0], OperandSize::Dword)
}

#[test]
fn vshuff64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM14)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 83, 205, 201, 35, 214, 34], OperandSize::Qword)
}

#[test]
fn vshuff64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 141, 206, 35, 23, 41], OperandSize::Qword)
}

#[test]
fn vshuff64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 197, 220, 35, 44, 145, 33], OperandSize::Qword)
}

