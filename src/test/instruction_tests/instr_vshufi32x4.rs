use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufi32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 203, 67, 250, 94], OperandSize::Dword)
}

#[test]
fn vshufi32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 2068112147, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 93, 203, 67, 164, 247, 19, 227, 68, 123, 18], OperandSize::Dword)
}

#[test]
fn vshufi32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 93, 222, 67, 36, 126, 126], OperandSize::Dword)
}

#[test]
fn vshufi32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM25)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 19, 85, 199, 67, 209, 8], OperandSize::Qword)
}

#[test]
fn vshufi32x4_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 374761826, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 69, 197, 67, 20, 85, 98, 105, 86, 22, 45], OperandSize::Qword)
}

#[test]
fn vshufi32x4_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFI32x4, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 5, 223, 67, 52, 139, 111], OperandSize::Qword)
}

