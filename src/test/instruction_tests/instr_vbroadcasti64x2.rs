use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 576081114, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 90, 180, 79, 218, 76, 86, 34], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 90, 47], OperandSize::Qword)
}

#[test]
fn vbroadcasti64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 90, 11], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X2, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectDisplaced(RBX, 651553891, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 206, 90, 187, 99, 236, 213, 38], OperandSize::Qword)
}

