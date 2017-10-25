use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttpd2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 122, 194], OperandSize::Dword)
}

fn vcvttpd2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1012893360, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 122, 164, 206, 176, 134, 95, 60], OperandSize::Dword)
}

fn vcvttpd2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 253, 143, 122, 231], OperandSize::Qword)
}

fn vcvttpd2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 137, 122, 38], OperandSize::Qword)
}

fn vcvttpd2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 122, 225], OperandSize::Dword)
}

fn vcvttpd2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 122, 20, 87], OperandSize::Dword)
}

fn vcvttpd2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 253, 170, 122, 246], OperandSize::Qword)
}

fn vcvttpd2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 169, 122, 36, 207], OperandSize::Qword)
}

fn vcvttpd2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 156, 122, 248], OperandSize::Dword)
}

fn vcvttpd2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 271744567, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 122, 20, 69, 55, 126, 50, 16], OperandSize::Dword)
}

fn vcvttpd2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 154, 122, 243], OperandSize::Qword)
}

fn vcvttpd2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2QQ, operand1: Some(Direct(ZMM24)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 253, 206, 122, 7], OperandSize::Qword)
}

