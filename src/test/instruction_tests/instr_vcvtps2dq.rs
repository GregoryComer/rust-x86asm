use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 250], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1674449458, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 180, 112, 50, 18, 206, 99], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 209], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RDX, 815805311, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 130, 127, 51, 160, 48], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 217], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1131918544, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 60, 141, 208, 180, 119, 67], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 253], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RBX, 2007882409, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 187, 169, 218, 173, 119], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 91, 216], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1350832299, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 91, 180, 65, 171, 16, 132, 80], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 125, 142, 91, 220], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1656914660, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 140, 91, 60, 117, 228, 130, 194, 98], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 91, 209], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 91, 4, 152], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 125, 170, 91, 210], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1721682025, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 175, 91, 156, 131, 105, 200, 158, 102], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 255, 91, 243], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDI, 1007867843, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 91, 191, 195, 215, 18, 60], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 125, 158, 91, 224], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectDisplaced(RDI, 20189780, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 125, 206, 91, 183, 84, 18, 52, 1], OperandSize::Qword)
}

