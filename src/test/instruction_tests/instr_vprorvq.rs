use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 20, 239], OperandSize::Dword)
}

#[test]
fn vprorvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 611780192, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 20, 140, 218, 96, 6, 119, 36], OperandSize::Dword)
}

#[test]
fn vprorvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1547870746, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 157, 20, 44, 253, 26, 162, 66, 92], OperandSize::Dword)
}

#[test]
fn vprorvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 205, 138, 20, 210], OperandSize::Qword)
}

#[test]
fn vprorvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectDisplaced(RSI, 580271130, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 245, 130, 20, 150, 26, 60, 150, 34], OperandSize::Qword)
}

#[test]
fn vprorvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 173, 153, 20, 3], OperandSize::Qword)
}

#[test]
fn vprorvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 20, 252], OperandSize::Dword)
}

#[test]
fn vprorvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 1200891869, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 170, 20, 134, 221, 39, 148, 71], OperandSize::Dword)
}

#[test]
fn vprorvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 189, 20, 28, 88], OperandSize::Dword)
}

#[test]
fn vprorvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 165, 170, 20, 219], OperandSize::Qword)
}

#[test]
fn vprorvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 669168654, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 189, 162, 20, 52, 77, 14, 180, 226, 39], OperandSize::Qword)
}

#[test]
fn vprorvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectDisplaced(RDI, 765110758, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 165, 178, 20, 175, 230, 169, 154, 45], OperandSize::Qword)
}

#[test]
fn vprorvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 20, 216], OperandSize::Dword)
}

#[test]
fn vprorvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 665823501, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 205, 20, 170, 13, 169, 175, 39], OperandSize::Dword)
}

#[test]
fn vprorvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1020950390, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 20, 140, 64, 118, 119, 218, 60], OperandSize::Dword)
}

#[test]
fn vprorvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 181, 206, 20, 223], OperandSize::Qword)
}

#[test]
fn vprorvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 203, 20, 23], OperandSize::Qword)
}

#[test]
fn vprorvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 702215900, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 20, 20, 205, 220, 246, 218, 41], OperandSize::Qword)
}

