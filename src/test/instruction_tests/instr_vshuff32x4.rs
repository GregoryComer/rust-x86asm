use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 85, 205, 35, 220, 15], OperandSize::Dword)
}

#[test]
fn vshuff32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 206, 35, 20, 199, 21], OperandSize::Dword)
}

#[test]
fn vshuff32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 44724668, Some(OperandSize::Dword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 101, 217, 35, 4, 197, 188, 113, 170, 2, 0], OperandSize::Dword)
}

#[test]
fn vshuff32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 69, 202, 35, 232, 83], OperandSize::Qword)
}

#[test]
fn vshuff32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 375963431, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 53, 206, 35, 60, 213, 39, 191, 104, 22, 88], OperandSize::Qword)
}

#[test]
fn vshuff32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF32x4, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 913204171, Some(OperandSize::Dword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 21, 221, 35, 156, 71, 203, 99, 110, 54, 75], OperandSize::Qword)
}

