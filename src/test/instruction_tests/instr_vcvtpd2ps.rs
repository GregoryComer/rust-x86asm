use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 220], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 17], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 240], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 38359857, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 148, 137, 49, 83, 73, 2], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 243], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1395946308, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 132, 206, 68, 115, 52, 83], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 201], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 51], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 90, 227], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 109430096, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 90, 132, 191, 80, 197, 133, 6], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 253, 143, 90, 242], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1487776622, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 253, 140, 90, 28, 221, 110, 171, 173, 88], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 90, 228], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 122975869, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 90, 140, 211, 125, 118, 84, 7], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 253, 174, 90, 193], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM16)), operand2: Some(IndirectDisplaced(RSI, 2021540388, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 174, 90, 134, 36, 66, 126, 120], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 186, 90, 200], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EAX, 533712859, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 90, 184, 219, 207, 207, 31], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 253, 223, 90, 199], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM14)), operand2: Some(IndirectDisplaced(RDX, 2040018185, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 253, 202, 90, 178, 9, 53, 152, 121], OperandSize::Qword)
}

