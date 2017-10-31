use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 226], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 0], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 204], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 445040109, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 12, 197, 237, 197, 134, 26], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 241], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 52, 248], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 207], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 32], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 91, 222], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1474133581, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 91, 28, 77, 77, 126, 221, 87], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 91, 242], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 944998875, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 142, 91, 148, 249, 219, 137, 83, 56], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 91, 207], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1567735557, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 91, 28, 197, 5, 191, 113, 93], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 125, 175, 91, 201], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectDisplaced(RDX, 166219592, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 125, 169, 91, 162, 72, 79, 232, 9], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 186, 91, 215], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 91, 1], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 125, 253, 91, 237], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1284374417, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 125, 202, 91, 180, 146, 145, 255, 141, 76], OperandSize::Qword)
}

