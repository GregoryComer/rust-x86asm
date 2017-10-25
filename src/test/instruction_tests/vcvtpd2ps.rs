use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 226], OperandSize::Dword)
}

fn vcvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 44, 255], OperandSize::Dword)
}

fn vcvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 210], OperandSize::Qword)
}

fn vcvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 36, 210], OperandSize::Qword)
}

fn vcvtpd2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 202], OperandSize::Dword)
}

fn vcvtpd2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 1913626808, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 163, 184, 160, 15, 114], OperandSize::Dword)
}

fn vcvtpd2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 247], OperandSize::Qword)
}

fn vcvtpd2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 31], OperandSize::Qword)
}

fn vcvtpd2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 90, 227], OperandSize::Dword)
}

fn vcvtpd2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1514017706, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 90, 140, 192, 170, 19, 62, 90], OperandSize::Dword)
}

fn vcvtpd2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 253, 139, 90, 218], OperandSize::Qword)
}

fn vcvtpd2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 42807695, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 253, 142, 90, 28, 205, 143, 49, 141, 2], OperandSize::Qword)
}

fn vcvtpd2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 90, 228], OperandSize::Dword)
}

fn vcvtpd2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 90, 20, 83], OperandSize::Dword)
}

fn vcvtpd2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 253, 174, 90, 238], OperandSize::Qword)
}

fn vcvtpd2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1990642661, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 253, 175, 90, 132, 195, 229, 203, 166, 118], OperandSize::Qword)
}

fn vcvtpd2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 255, 90, 226], OperandSize::Dword)
}

fn vcvtpd2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 90, 32], OperandSize::Dword)
}

fn vcvtpd2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 253, 223, 90, 254], OperandSize::Qword)
}

fn vcvtpd2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 253, 205, 90, 28, 122], OperandSize::Qword)
}

