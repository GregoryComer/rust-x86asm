use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vporq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 143, 235, 204], OperandSize::Dword)
}

#[test]
fn vporq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1155602811, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 143, 235, 36, 93, 123, 25, 225, 68], OperandSize::Dword)
}

#[test]
fn vporq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1531016430, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 153, 235, 156, 202, 238, 116, 65, 91], OperandSize::Dword)
}

#[test]
fn vporq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 173, 139, 235, 207], OperandSize::Qword)
}

#[test]
fn vporq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1553133352, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 197, 138, 235, 172, 159, 40, 239, 146, 92], OperandSize::Qword)
}

#[test]
fn vporq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1183869921, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 157, 155, 235, 188, 248, 225, 107, 144, 70], OperandSize::Qword)
}

#[test]
fn vporq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 171, 235, 240], OperandSize::Dword)
}

#[test]
fn vporq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 170, 235, 30], OperandSize::Dword)
}

#[test]
fn vporq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1028768347, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 188, 235, 4, 93, 91, 194, 81, 61], OperandSize::Dword)
}

#[test]
fn vporq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 165, 161, 235, 201], OperandSize::Qword)
}

#[test]
fn vporq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1876186775, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 133, 162, 235, 172, 71, 151, 86, 212, 111], OperandSize::Qword)
}

#[test]
fn vporq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 133, 178, 235, 14], OperandSize::Qword)
}

#[test]
fn vporq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 203, 235, 245], OperandSize::Dword)
}

#[test]
fn vporq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 131714274, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 203, 235, 172, 247, 226, 204, 217, 7], OperandSize::Dword)
}

#[test]
fn vporq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 986980752, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 223, 235, 60, 133, 144, 33, 212, 58], OperandSize::Dword)
}

#[test]
fn vporq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 149, 207, 235, 244], OperandSize::Qword)
}

#[test]
fn vporq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RCX, 332144255, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 157, 201, 235, 129, 127, 30, 204, 19], OperandSize::Qword)
}

#[test]
fn vporq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 220, 235, 32], OperandSize::Qword)
}

