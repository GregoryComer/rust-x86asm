use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 137, 252], OperandSize::Dword)
}

#[test]
fn vpexpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 137, 15], OperandSize::Dword)
}

#[test]
fn vpexpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 253, 143, 137, 252], OperandSize::Qword)
}

#[test]
fn vpexpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectDisplaced(RCX, 1141373625, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 143, 137, 185, 185, 250, 7, 68], OperandSize::Qword)
}

#[test]
fn vpexpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 137, 223], OperandSize::Dword)
}

#[test]
fn vpexpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 137, 60, 209], OperandSize::Dword)
}

#[test]
fn vpexpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 253, 172, 137, 194], OperandSize::Qword)
}

#[test]
fn vpexpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 137, 60, 74], OperandSize::Qword)
}

#[test]
fn vpexpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 137, 203], OperandSize::Dword)
}

#[test]
fn vpexpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDX, 739061974, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 137, 170, 214, 48, 13, 44], OperandSize::Dword)
}

#[test]
fn vpexpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 253, 205, 137, 197], OperandSize::Qword)
}

#[test]
fn vpexpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1144471514, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 137, 4, 213, 218, 63, 55, 68], OperandSize::Qword)
}

