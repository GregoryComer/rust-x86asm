use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 101, 201, 35, 193, 27], OperandSize::Dword)
}

#[test]
fn vshuff32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 203, 35, 10, 107], OperandSize::Dword)
}

#[test]
fn vshuff32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 1825713404, Some(OperandSize::Dword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 221, 35, 153, 252, 44, 210, 108, 78], OperandSize::Dword)
}

#[test]
fn vshuff32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM13)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 61, 194, 35, 205, 68], OperandSize::Qword)
}

#[test]
fn vshuff32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1207324258, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 125, 207, 35, 164, 73, 98, 78, 246, 71, 25], OperandSize::Qword)
}

#[test]
fn vshuff32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1464612193, Some(OperandSize::Dword), None)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 29, 217, 35, 28, 141, 97, 53, 76, 87, 19], OperandSize::Qword)
}

