use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 118, 245], OperandSize::Dword)
}

#[test]
fn vpermi2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 118, 56], OperandSize::Dword)
}

#[test]
fn vpermi2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 159, 118, 49], OperandSize::Dword)
}

#[test]
fn vpermi2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 165, 142, 118, 201], OperandSize::Qword)
}

#[test]
fn vpermi2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 221, 142, 118, 36, 191], OperandSize::Qword)
}

#[test]
fn vpermi2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 249574771, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 165, 149, 118, 20, 117, 115, 53, 224, 14], OperandSize::Qword)
}

#[test]
fn vpermi2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 118, 206], OperandSize::Dword)
}

#[test]
fn vpermi2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1349940702, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 118, 28, 157, 222, 117, 118, 80], OperandSize::Dword)
}

#[test]
fn vpermi2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ESI, 1902985606, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 190, 118, 150, 134, 65, 109, 113], OperandSize::Dword)
}

#[test]
fn vpermi2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 157, 173, 118, 211], OperandSize::Qword)
}

#[test]
fn vpermi2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RBX, 1661664880, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 245, 161, 118, 155, 112, 254, 10, 99], OperandSize::Qword)
}

#[test]
fn vpermi2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1009779207, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 133, 185, 118, 140, 87, 7, 2, 48, 60], OperandSize::Qword)
}

#[test]
fn vpermi2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 118, 200], OperandSize::Dword)
}

#[test]
fn vpermi2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 1720614929, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 118, 185, 17, 128, 142, 102], OperandSize::Dword)
}

#[test]
fn vpermi2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1500770432, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 118, 156, 73, 128, 240, 115, 89], OperandSize::Dword)
}

#[test]
fn vpermi2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 165, 195, 118, 244], OperandSize::Qword)
}

#[test]
fn vpermi2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RBX, 1168153749, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 229, 205, 118, 179, 149, 156, 160, 69], OperandSize::Qword)
}

#[test]
fn vpermi2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1389331479, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 213, 118, 164, 211, 23, 132, 207, 82], OperandSize::Qword)
}

