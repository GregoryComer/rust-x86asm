use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 198], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 51], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 238], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 756883970, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 172, 186, 2, 34, 29, 45], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 200], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1661780881, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 60, 157, 145, 195, 12, 99], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 233], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 4, 190], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 230, 217], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 230, 20, 201], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 253, 141, 230, 229], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM25)), operand2: Some(IndirectDisplaced(RSI, 117544590, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 253, 142, 230, 142, 142, 150, 1, 7], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 230, 218], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EBX, 161403977, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 230, 187, 73, 212, 158, 9], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 253, 169, 230, 223], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 480624970, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 170, 230, 20, 77, 74, 193, 165, 28], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 154, 230, 248], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 954260230, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 230, 36, 117, 6, 219, 224, 56], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 154, 230, 247], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 253, 205, 230, 36, 90], OperandSize::Qword)
}

