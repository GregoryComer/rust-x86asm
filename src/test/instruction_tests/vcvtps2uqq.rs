use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 121, 210], OperandSize::Dword)
}

fn vcvtps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1478787531, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 121, 4, 77, 203, 129, 36, 88], OperandSize::Dword)
}

fn vcvtps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 125, 138, 121, 230], OperandSize::Qword)
}

fn vcvtps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 954168128, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 143, 121, 44, 141, 64, 115, 223, 56], OperandSize::Qword)
}

fn vcvtps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 121, 221], OperandSize::Dword)
}

fn vcvtps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1626615170, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 121, 188, 249, 130, 45, 244, 96], OperandSize::Dword)
}

fn vcvtps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 125, 170, 121, 231], OperandSize::Qword)
}

fn vcvtps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 172, 121, 36, 153], OperandSize::Qword)
}

fn vcvtps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 223, 121, 201], OperandSize::Dword)
}

fn vcvtps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EAX, 1408034500, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 121, 136, 196, 230, 236, 83], OperandSize::Dword)
}

fn vcvtps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 125, 255, 121, 247], OperandSize::Qword)
}

fn vcvtps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 680479972, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 121, 172, 182, 228, 76, 143, 40], OperandSize::Qword)
}

