use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 91, 27], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectDisplaced(RAX, 301743112, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 206, 91, 152, 8, 60, 252, 17], OperandSize::Qword)
}

