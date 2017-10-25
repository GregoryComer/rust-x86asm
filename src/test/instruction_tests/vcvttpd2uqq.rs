use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttpd2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 120, 237], OperandSize::Dword)
}

fn vcvttpd2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 439134269, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 120, 188, 185, 61, 168, 44, 26], OperandSize::Dword)
}

fn vcvttpd2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 253, 138, 120, 214], OperandSize::Qword)
}

fn vcvttpd2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(XMM25)), operand2: Some(IndirectDisplaced(RDI, 1802086302, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 253, 138, 120, 143, 158, 167, 105, 107], OperandSize::Qword)
}

fn vcvttpd2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 120, 196], OperandSize::Dword)
}

fn vcvttpd2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1328394870, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 120, 4, 141, 118, 178, 45, 79], OperandSize::Dword)
}

fn vcvttpd2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 120, 220], OperandSize::Qword)
}

fn vcvttpd2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 174, 120, 12, 134], OperandSize::Qword)
}

fn vcvttpd2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 157, 120, 204], OperandSize::Dword)
}

fn vcvttpd2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 120, 4, 142], OperandSize::Dword)
}

fn vcvttpd2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 155, 120, 206], OperandSize::Qword)
}

fn vcvttpd2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPD2UQQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 207, 120, 12, 145], OperandSize::Qword)
}

