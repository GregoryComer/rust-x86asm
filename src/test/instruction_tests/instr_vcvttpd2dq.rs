use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 222], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 2104419508, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 161, 180, 228, 110, 125], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 199], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 230, 60, 64], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 252], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 819308041, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 188, 66, 9, 166, 213, 48], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 217], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 230, 55], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 230, 194], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 214764995, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 230, 52, 69, 195, 13, 205, 12], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 230, 238], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 504664378, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 230, 156, 153, 58, 145, 20, 30], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 230, 245], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 230, 55], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 253, 172, 230, 213], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(XMM30)), operand2: Some(IndirectDisplaced(RBX, 1890758780, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 253, 174, 230, 179, 124, 176, 178, 112], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 154, 230, 202], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 230, 44, 80], OperandSize::Dword)
}

#[test]
fn vcvttpd2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 253, 154, 230, 245], OperandSize::Qword)
}

#[test]
fn vcvttpd2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2DQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 690592911, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 203, 230, 36, 245, 143, 156, 41, 41], OperandSize::Qword)
}

