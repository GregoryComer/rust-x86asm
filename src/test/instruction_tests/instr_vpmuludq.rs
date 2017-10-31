use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 244, 239], OperandSize::Dword)
}

#[test]
fn vpmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 244, 12, 122], OperandSize::Dword)
}

#[test]
fn vpmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 244, 234], OperandSize::Qword)
}

#[test]
fn vpmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 475612503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 244, 44, 181, 87, 69, 89, 28], OperandSize::Qword)
}

#[test]
fn vpmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 244, 219], OperandSize::Dword)
}

#[test]
fn vpmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 2018283145, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 244, 36, 181, 137, 142, 76, 120], OperandSize::Dword)
}

#[test]
fn vpmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 244, 244], OperandSize::Qword)
}

#[test]
fn vpmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 244, 44, 183], OperandSize::Qword)
}

#[test]
fn vpmuludq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 141, 244, 207], OperandSize::Dword)
}

#[test]
fn vpmuludq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1036343830, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 143, 244, 155, 22, 90, 197, 61], OperandSize::Dword)
}

#[test]
fn vpmuludq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 946439009, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 154, 244, 140, 139, 97, 131, 105, 56], OperandSize::Dword)
}

#[test]
fn vpmuludq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 205, 141, 244, 212], OperandSize::Qword)
}

#[test]
fn vpmuludq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1503367757, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 134, 244, 52, 93, 77, 146, 155, 89], OperandSize::Qword)
}

#[test]
fn vpmuludq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 149, 158, 244, 28, 137], OperandSize::Qword)
}

#[test]
fn vpmuludq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 173, 244, 244], OperandSize::Dword)
}

#[test]
fn vpmuludq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 169, 244, 17], OperandSize::Dword)
}

#[test]
fn vpmuludq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 658473738, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 191, 244, 140, 87, 10, 131, 63, 39], OperandSize::Dword)
}

#[test]
fn vpmuludq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 173, 164, 244, 239], OperandSize::Qword)
}

#[test]
fn vpmuludq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RSI, 997122181, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 189, 166, 244, 174, 133, 224, 110, 59], OperandSize::Qword)
}

#[test]
fn vpmuludq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 183, 244, 57], OperandSize::Qword)
}

#[test]
fn vpmuludq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 244, 227], OperandSize::Dword)
}

#[test]
fn vpmuludq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1490626574, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 202, 244, 52, 253, 14, 40, 217, 88], OperandSize::Dword)
}

#[test]
fn vpmuludq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ECX, 1668714184, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 219, 244, 153, 200, 142, 118, 99], OperandSize::Dword)
}

#[test]
fn vpmuludq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 213, 202, 244, 234], OperandSize::Qword)
}

#[test]
fn vpmuludq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 149, 199, 244, 42], OperandSize::Qword)
}

#[test]
fn vpmuludq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 181, 212, 244, 44, 153], OperandSize::Qword)
}

