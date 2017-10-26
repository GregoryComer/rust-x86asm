use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 141, 120, 211], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 138, 120, 9], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 252, 139, 120, 220], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 953118014, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 252, 138, 120, 156, 186, 62, 109, 207, 56], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 173, 120, 192], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 445777572, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 172, 120, 188, 218, 164, 6, 146, 26], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 170, 120, 254], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 599820255, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 252, 172, 120, 172, 135, 223, 135, 192, 35], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 154, 120, 214], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 202, 120, 60, 207], OperandSize::Dword)
}

#[test]
fn vcvttpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 252, 155, 120, 240], OperandSize::Qword)
}

#[test]
fn vcvttpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UDQ, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 201, 120, 34], OperandSize::Qword)
}

