use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EAX, 2107703894, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 91, 136, 86, 2, 161, 125], OperandSize::Dword)
}

#[test]
fn vbroadcasti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1596671771, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 203, 91, 28, 189, 27, 71, 43, 95], OperandSize::Qword)
}

