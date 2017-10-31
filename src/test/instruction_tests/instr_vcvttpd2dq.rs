use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 207], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1781586917, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 148, 193, 229, 219, 48, 106], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 241], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RAX, 291384628, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 160, 52, 45, 94, 17], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 212], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 2104358843, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 44, 141, 187, 247, 109, 125], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 194], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 52, 95], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 230, 219], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 230, 60, 222], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 253, 141, 230, 212], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM28)), operand2: Some(IndirectDisplaced(RDX, 400465672, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 253, 139, 230, 162, 8, 159, 222, 23], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 230, 223], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 19511015, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 230, 60, 85, 231, 182, 41, 1], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 253, 172, 230, 240], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1198953118, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 175, 230, 140, 129, 158, 146, 118, 71], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 155, 230, 229], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(ECX, 588803316, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 230, 161, 244, 108, 24, 35], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 253, 156, 230, 222], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 201, 230, 60, 223], OperandSize::Qword)
}

