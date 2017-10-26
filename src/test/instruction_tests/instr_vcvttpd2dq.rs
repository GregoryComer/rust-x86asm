use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 200], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1984360293, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 172, 152, 101, 239, 70, 118], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 208], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 48], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 250], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDI, 1082429234, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 143, 50, 143, 132, 64], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 203], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 55], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 230, 193], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 230, 62], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 137, 230, 194], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 200024545, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 230, 52, 245, 225, 33, 236, 11], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 230, 201], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 467044173, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 230, 60, 149, 77, 135, 214, 27], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 253, 172, 230, 221], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1263548772, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 175, 230, 36, 141, 100, 57, 80, 75], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 156, 230, 214], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 230, 22], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 253, 157, 230, 253], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RAX, 1187577332, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 203, 230, 160, 244, 253, 200, 70], OperandSize::Qword)
}

