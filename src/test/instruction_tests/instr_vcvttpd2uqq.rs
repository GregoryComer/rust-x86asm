use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 120, 222], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1694056880, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 120, 132, 94, 176, 65, 249, 100], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 253, 139, 120, 200], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 137, 120, 44, 254], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 120, 196], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 899609204, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 120, 156, 94, 116, 242, 158, 53], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 253, 171, 120, 225], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RDX, 568767424, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 120, 154, 192, 179, 230, 33], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 159, 120, 219], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 120, 47], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 156, 120, 239], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1652898088, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 120, 172, 120, 40, 57, 133, 98], OperandSize::Qword)
}

