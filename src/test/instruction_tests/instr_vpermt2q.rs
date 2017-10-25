use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 141, 126, 237], OperandSize::Dword)
}

#[test]
fn vpermt2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1089007037, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 126, 180, 222, 189, 237, 232, 64], OperandSize::Dword)
}

#[test]
fn vpermt2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 140311080, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 156, 126, 172, 208, 40, 250, 92, 8], OperandSize::Dword)
}

#[test]
fn vpermt2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 189, 130, 126, 248], OperandSize::Qword)
}

#[test]
fn vpermt2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 880870368, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 149, 138, 126, 4, 125, 224, 3, 129, 52], OperandSize::Qword)
}

#[test]
fn vpermt2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RDI, 36788114, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 173, 154, 126, 135, 146, 87, 49, 2], OperandSize::Qword)
}

#[test]
fn vpermt2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 170, 126, 239], OperandSize::Dword)
}

#[test]
fn vpermt2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 126, 20, 203], OperandSize::Dword)
}

#[test]
fn vpermt2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 190, 126, 27], OperandSize::Dword)
}

#[test]
fn vpermt2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 133, 166, 126, 243], OperandSize::Qword)
}

#[test]
fn vpermt2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 862360546, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 245, 161, 126, 148, 145, 226, 147, 102, 51], OperandSize::Qword)
}

#[test]
fn vpermt2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 795755400, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 197, 187, 126, 132, 190, 136, 67, 110, 47], OperandSize::Qword)
}

#[test]
fn vpermt2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 126, 243], OperandSize::Dword)
}

#[test]
fn vpermt2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1290981694, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 206, 126, 132, 80, 62, 209, 242, 76], OperandSize::Dword)
}

#[test]
fn vpermt2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 126, 6], OperandSize::Dword)
}

#[test]
fn vpermt2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 197, 194, 126, 195], OperandSize::Qword)
}

#[test]
fn vpermt2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1728863304, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 133, 197, 126, 12, 141, 72, 92, 12, 103], OperandSize::Qword)
}

#[test]
fn vpermt2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 613290242, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 217, 126, 132, 130, 2, 17, 142, 36], OperandSize::Qword)
}

