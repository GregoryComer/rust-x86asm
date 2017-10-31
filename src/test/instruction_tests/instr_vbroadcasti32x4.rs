use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EAX, 227162414, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 90, 136, 46, 57, 138, 13], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM31)), operand2: Some(IndirectDisplaced(RDX, 622209229, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 90, 186, 205, 40, 22, 37], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 465507030, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 90, 164, 138, 214, 18, 191, 27], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM16)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 90, 3], OperandSize::Qword)
}

