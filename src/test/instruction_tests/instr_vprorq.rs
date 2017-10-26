use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 143, 114, 199, 122], OperandSize::Dword)
}

#[test]
fn vprorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1665188269, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 143, 114, 132, 152, 173, 193, 64, 99, 29], OperandSize::Dword)
}

#[test]
fn vprorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDX, 517284130, Some(OperandSize::Qword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 154, 114, 130, 34, 33, 213, 30, 70], OperandSize::Dword)
}

#[test]
fn vprorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM31)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 205, 133, 114, 199, 62], OperandSize::Qword)
}

#[test]
fn vprorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1639163973, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 189, 132, 114, 132, 122, 69, 168, 179, 97, 31], OperandSize::Qword)
}

#[test]
fn vprorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RCX, 63852000, Some(OperandSize::Qword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 155, 114, 129, 224, 77, 206, 3, 26], OperandSize::Qword)
}

#[test]
fn vprorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 171, 114, 199, 26], OperandSize::Dword)
}

#[test]
fn vprorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 114, 2, 81], OperandSize::Dword)
}

#[test]
fn vprorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 652773393, Some(OperandSize::Qword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 189, 114, 4, 189, 17, 136, 232, 38, 57], OperandSize::Dword)
}

#[test]
fn vprorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 114, 195, 87], OperandSize::Qword)
}

#[test]
fn vprorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM28)), operand2: Some(IndirectDisplaced(RBX, 33223981, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 157, 166, 114, 131, 45, 245, 250, 1, 51], OperandSize::Qword)
}

#[test]
fn vprorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 183, 114, 4, 145, 40], OperandSize::Qword)
}

#[test]
fn vprorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 202, 114, 199, 41], OperandSize::Dword)
}

#[test]
fn vprorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 203, 114, 1, 90], OperandSize::Dword)
}

#[test]
fn vprorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDI, 1213676212, Some(OperandSize::Qword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 219, 114, 135, 180, 58, 87, 72, 88], OperandSize::Dword)
}

#[test]
fn vprorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM23)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 197, 195, 114, 199, 13], OperandSize::Qword)
}

#[test]
fn vprorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectDisplaced(RDI, 1560578070, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 194, 114, 135, 22, 136, 4, 93, 104], OperandSize::Qword)
}

#[test]
fn vprorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 173, 209, 114, 4, 243, 86], OperandSize::Qword)
}

