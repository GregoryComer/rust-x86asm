use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 118, 234], OperandSize::Dword)
}

#[test]
fn vpermi2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EAX, 1382301645, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 137, 118, 184, 205, 63, 100, 82], OperandSize::Dword)
}

#[test]
fn vpermi2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 118, 44, 145], OperandSize::Dword)
}

#[test]
fn vpermi2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 221, 138, 118, 231], OperandSize::Qword)
}

#[test]
fn vpermi2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 229, 143, 118, 36, 145], OperandSize::Qword)
}

#[test]
fn vpermi2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 157, 146, 118, 60, 128], OperandSize::Qword)
}

#[test]
fn vpermi2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 118, 221], OperandSize::Dword)
}

#[test]
fn vpermi2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1471392876, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 118, 172, 206, 108, 172, 179, 87], OperandSize::Dword)
}

#[test]
fn vpermi2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 185, 118, 36, 142], OperandSize::Dword)
}

#[test]
fn vpermi2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 118, 214], OperandSize::Qword)
}

#[test]
fn vpermi2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 157, 162, 118, 30], OperandSize::Qword)
}

#[test]
fn vpermi2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 831661632, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 181, 118, 132, 73, 64, 38, 146, 49], OperandSize::Qword)
}

#[test]
fn vpermi2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 207, 118, 219], OperandSize::Dword)
}

#[test]
fn vpermi2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1052719800, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 203, 118, 44, 141, 184, 58, 191, 62], OperandSize::Dword)
}

#[test]
fn vpermi2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 220, 118, 20, 66], OperandSize::Dword)
}

#[test]
fn vpermi2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 141, 205, 118, 206], OperandSize::Qword)
}

#[test]
fn vpermi2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 185485598, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 141, 197, 118, 172, 246, 30, 73, 14, 11], OperandSize::Qword)
}

#[test]
fn vpermi2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1722528175, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 149, 211, 118, 60, 181, 175, 177, 171, 102], OperandSize::Qword)
}

