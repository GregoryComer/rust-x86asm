use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 195], OperandSize::Dword)
}

fn vcvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 2001059278, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 172, 78, 206, 189, 69, 119], OperandSize::Dword)
}

fn vcvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 226], OperandSize::Qword)
}

fn vcvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 1], OperandSize::Qword)
}

fn vcvtps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 194], OperandSize::Dword)
}

fn vcvtps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EBX, 819493451, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 147, 75, 122, 216, 48], OperandSize::Dword)
}

fn vcvtps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 206], OperandSize::Qword)
}

fn vcvtps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 60, 150], OperandSize::Qword)
}

fn vcvtps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 91, 226], OperandSize::Dword)
}

fn vcvtps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 479127279, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 91, 20, 93, 239, 230, 142, 28], OperandSize::Dword)
}

fn vcvtps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 125, 141, 91, 221], OperandSize::Qword)
}

fn vcvtps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 125, 139, 91, 28, 91], OperandSize::Qword)
}

fn vcvtps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 91, 194], OperandSize::Dword)
}

fn vcvtps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 91, 36, 254], OperandSize::Dword)
}

fn vcvtps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 125, 173, 91, 222], OperandSize::Qword)
}

fn vcvtps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM30)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 172, 91, 51], OperandSize::Qword)
}

fn vcvtps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 249, 91, 207], OperandSize::Dword)
}

fn vcvtps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 91, 44, 246], OperandSize::Dword)
}

fn vcvtps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 125, 255, 91, 217], OperandSize::Qword)
}

fn vcvtps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(RAX, 1233691557, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 91, 184, 165, 163, 136, 73], OperandSize::Qword)
}

