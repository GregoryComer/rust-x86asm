use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtpd2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 143, 121, 223], OperandSize::Dword)
}

fn vcvtpd2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 358277391, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 140, 121, 135, 15, 225, 90, 21], OperandSize::Dword)
}

fn vcvtpd2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 252, 141, 121, 250], OperandSize::Qword)
}

fn vcvtpd2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 2126907941, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 252, 139, 121, 172, 130, 37, 10, 198, 126], OperandSize::Qword)
}

fn vcvtpd2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 252, 169, 121, 197], OperandSize::Dword)
}

fn vcvtpd2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 2087753904, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 174, 121, 175, 176, 152, 112, 124], OperandSize::Dword)
}

fn vcvtpd2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 252, 174, 121, 206], OperandSize::Qword)
}

fn vcvtpd2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(XMM21)), operand2: Some(IndirectDisplaced(RSI, 1664782057, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 252, 175, 121, 174, 233, 142, 58, 99], OperandSize::Qword)
}

fn vcvtpd2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 250, 121, 204], OperandSize::Dword)
}

fn vcvtpd2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 205, 121, 52, 127], OperandSize::Dword)
}

fn vcvtpd2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 252, 189, 121, 223], OperandSize::Qword)
}

fn vcvtpd2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2UDQ, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1795081037, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 252, 203, 121, 20, 245, 77, 195, 254, 106], OperandSize::Qword)
}

