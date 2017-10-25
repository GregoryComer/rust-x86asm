use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 123, 245], OperandSize::Dword)
}

fn vcvtpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 512032535, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 123, 140, 177, 23, 255, 132, 30], OperandSize::Dword)
}

fn vcvtpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 253, 139, 123, 221], OperandSize::Qword)
}

fn vcvtpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 253, 143, 123, 20, 214], OperandSize::Qword)
}

fn vcvtpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 123, 203], OperandSize::Dword)
}

fn vcvtpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1713922455, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 123, 172, 129, 151, 97, 40, 102], OperandSize::Dword)
}

fn vcvtpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 253, 173, 123, 200], OperandSize::Qword)
}

fn vcvtpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 65160090, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 175, 123, 4, 69, 154, 67, 226, 3], OperandSize::Qword)
}

fn vcvtpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 223, 123, 205], OperandSize::Dword)
}

fn vcvtpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 123, 20, 190], OperandSize::Dword)
}

fn vcvtpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 253, 254, 123, 253], OperandSize::Qword)
}

fn vcvtpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2QQ, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1628433046, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 253, 203, 123, 132, 78, 150, 234, 15, 97], OperandSize::Qword)
}

