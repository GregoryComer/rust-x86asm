use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 122, 254], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 611885406, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 122, 60, 69, 94, 161, 120, 36], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 253, 141, 122, 196], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1721829498, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 141, 122, 132, 241, 122, 8, 161, 102], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 122, 204], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(ECX, 475561757, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 122, 185, 29, 127, 88, 28], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 253, 174, 122, 215], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM19)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 170, 122, 30], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 155, 122, 214], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1372233587, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 122, 36, 245, 115, 159, 202, 81], OperandSize::Dword)
}

#[test]
fn vcvttpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 253, 157, 122, 236], OperandSize::Qword)
}

#[test]
fn vcvttpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(RSI, 1301578380, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 122, 174, 140, 130, 148, 77], OperandSize::Qword)
}

