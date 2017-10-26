use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 247], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 22], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 212], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1760712118, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 148, 190, 182, 85, 242, 104], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 192], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 24], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 251], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 469790591, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 12, 77, 127, 111, 0, 28], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 91, 241], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 2133747129, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 91, 52, 213, 185, 101, 46, 127], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 126, 139, 91, 210], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 91, 60, 81], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 91, 193], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 117659822, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 91, 60, 125, 174, 88, 3, 7], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 126, 173, 91, 254], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1829950813, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 91, 4, 213, 93, 213, 18, 109], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 159, 91, 235], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 416901390, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 91, 148, 79, 14, 105, 217, 24], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 126, 157, 91, 202], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 126, 204, 91, 20, 64], OperandSize::Qword)
}

