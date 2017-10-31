use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 251], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1258161158, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 140, 193, 6, 4, 254, 74], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 195], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 869375514, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 177, 26, 158, 209, 51], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 192], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EBX, 2025706998, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 163, 246, 213, 189, 120], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 211], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 52, 80], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 91, 195], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 91, 49], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 126, 139, 91, 209], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 2121010943, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 91, 28, 133, 255, 14, 108, 126], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 91, 199], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ESI, 466472228, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 91, 134, 36, 205, 205, 27], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 126, 174, 91, 201], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1991420772, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 170, 91, 44, 157, 100, 171, 178, 118], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 154, 91, 207], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 91, 44, 159], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 126, 153, 91, 227], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 91, 4, 146], OperandSize::Qword)
}

