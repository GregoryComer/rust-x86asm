use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 21, 233], OperandSize::Dword)
}

#[test]
fn vprolvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1617384683, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 21, 183, 235, 84, 103, 96], OperandSize::Dword)
}

#[test]
fn vprolvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 159, 21, 20, 246], OperandSize::Dword)
}

#[test]
fn vprolvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 221, 135, 21, 245], OperandSize::Qword)
}

#[test]
fn vprolvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 229, 133, 21, 44, 206], OperandSize::Qword)
}

#[test]
fn vprolvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 213, 159, 21, 25], OperandSize::Qword)
}

#[test]
fn vprolvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 21, 234], OperandSize::Dword)
}

#[test]
fn vprolvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 21, 23], OperandSize::Dword)
}

#[test]
fn vprolvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 412059699, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 185, 21, 132, 206, 51, 136, 143, 24], OperandSize::Dword)
}

#[test]
fn vprolvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 141, 164, 21, 251], OperandSize::Qword)
}

#[test]
fn vprolvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1550187599, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 165, 169, 21, 132, 254, 79, 252, 101, 92], OperandSize::Qword)
}

#[test]
fn vprolvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RAX, 938920675, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 133, 183, 21, 176, 227, 202, 246, 55], OperandSize::Qword)
}

#[test]
fn vprolvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 203, 21, 230], OperandSize::Dword)
}

#[test]
fn vprolvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1099508591, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 201, 21, 52, 197, 111, 43, 137, 65], OperandSize::Dword)
}

#[test]
fn vprolvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 223, 21, 4, 158], OperandSize::Dword)
}

#[test]
fn vprolvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 245, 199, 21, 196], OperandSize::Qword)
}

#[test]
fn vprolvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1628660068, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 149, 199, 21, 132, 74, 100, 97, 19, 97], OperandSize::Qword)
}

#[test]
fn vprolvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1594986989, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 210, 21, 12, 117, 237, 145, 17, 95], OperandSize::Qword)
}

