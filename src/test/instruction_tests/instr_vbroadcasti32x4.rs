use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1380566987, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 90, 20, 181, 203, 199, 73, 82], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM12)), operand2: Some(IndirectDisplaced(RDI, 605834935, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 171, 90, 167, 183, 78, 28, 36], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 90, 60, 248], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM14)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 90, 51], OperandSize::Qword)
}

