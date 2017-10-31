use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 120, 242], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 1053303972, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 120, 191, 164, 36, 200, 62], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 253, 141, 120, 233], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM10)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 140, 120, 16], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 120, 200], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 120, 30], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 253, 173, 120, 218], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RDI, 67667828, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 120, 191, 116, 135, 8, 4], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 155, 120, 254], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1749090696, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 120, 164, 136, 136, 1, 65, 104], OperandSize::Dword)
}

#[test]
fn vcvttpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 253, 159, 120, 198], OperandSize::Qword)
}

#[test]
fn vcvttpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RDX, 1581341637, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 120, 146, 197, 91, 65, 94], OperandSize::Qword)
}

