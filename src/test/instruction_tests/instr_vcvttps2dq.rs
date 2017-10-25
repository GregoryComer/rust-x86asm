use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 229], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1059433490, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 164, 80, 18, 172, 37, 63], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 215], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 39], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 199], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 58], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 199], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1746798291, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 12, 189, 211, 6, 30, 104], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 91, 219], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 480936885, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 91, 140, 151, 181, 131, 170, 28], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 126, 141, 91, 224], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RCX, 541929386, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 91, 145, 170, 47, 77, 32], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 91, 209], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 163027692, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 91, 28, 221, 236, 154, 183, 9], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 126, 174, 91, 225], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1617900571, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 126, 171, 91, 20, 205, 27, 52, 111, 96], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 157, 91, 233], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 91, 9], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 153, 91, 236], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1209671395, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 91, 140, 250, 227, 30, 26, 72], OperandSize::Qword)
}

