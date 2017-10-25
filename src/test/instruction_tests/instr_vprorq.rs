use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 138, 114, 193, 104], OperandSize::Dword)
}

#[test]
fn vprorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2003037825, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 114, 4, 197, 129, 238, 99, 119, 83], OperandSize::Dword)
}

#[test]
fn vprorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 155, 114, 4, 192, 51], OperandSize::Dword)
}

#[test]
fn vprorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM12)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 213, 141, 114, 196, 58], OperandSize::Qword)
}

#[test]
fn vprorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM23)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 131, 114, 7, 81], OperandSize::Qword)
}

#[test]
fn vprorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 159, 114, 4, 195, 20], OperandSize::Qword)
}

#[test]
fn vprorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 171, 114, 192, 86], OperandSize::Dword)
}

#[test]
fn vprorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 175, 114, 4, 115, 86], OperandSize::Dword)
}

#[test]
fn vprorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 185, 114, 4, 114, 10], OperandSize::Dword)
}

#[test]
fn vprorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 173, 171, 114, 193, 30], OperandSize::Qword)
}

#[test]
fn vprorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1836361473, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 141, 171, 114, 4, 245, 1, 167, 116, 109, 40], OperandSize::Qword)
}

#[test]
fn vprorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 165, 178, 114, 4, 143, 84], OperandSize::Qword)
}

#[test]
fn vprorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 202, 114, 199, 97], OperandSize::Dword)
}

#[test]
fn vprorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 204, 114, 2, 43], OperandSize::Dword)
}

#[test]
fn vprorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EDI, 2104555158, Some(OperandSize::Qword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 220, 114, 135, 150, 246, 112, 125, 105], OperandSize::Dword)
}

#[test]
fn vprorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM13)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 181, 195, 114, 197, 51], OperandSize::Qword)
}

#[test]
fn vprorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 181, 196, 114, 4, 121, 16], OperandSize::Qword)
}

#[test]
fn vprorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 855806995, Some(OperandSize::Qword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 189, 209, 114, 4, 77, 19, 148, 2, 51, 32], OperandSize::Qword)
}

