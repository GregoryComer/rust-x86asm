use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 90, 36, 243], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 198255812, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 90, 12, 125, 196, 36, 209, 11], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EDX, 1413429012, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 90, 146, 20, 55, 63, 84], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32X4, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 90, 27], OperandSize::Qword)
}

