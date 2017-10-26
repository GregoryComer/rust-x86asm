use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 235, 232], OperandSize::Dword)
}

#[test]
fn vpord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 235, 19], OperandSize::Dword)
}

#[test]
fn vpord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 158, 235, 20, 119], OperandSize::Dword)
}

#[test]
fn vpord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 5, 139, 235, 229], OperandSize::Qword)
}

#[test]
fn vpord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 666258930, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 21, 133, 235, 44, 213, 242, 77, 182, 39], OperandSize::Qword)
}

#[test]
fn vpord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1145272735, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 153, 235, 164, 66, 159, 121, 67, 68], OperandSize::Qword)
}

#[test]
fn vpord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 235, 205], OperandSize::Dword)
}

#[test]
fn vpord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ESI, 1673555693, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 235, 142, 237, 110, 192, 99], OperandSize::Dword)
}

#[test]
fn vpord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 85, 190, 235, 59], OperandSize::Dword)
}

#[test]
fn vpord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 53, 170, 235, 253], OperandSize::Qword)
}

#[test]
fn vpord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 45, 164, 235, 52, 185], OperandSize::Qword)
}

#[test]
fn vpord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RSI, 1430181501, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 13, 186, 235, 174, 125, 214, 62, 85], OperandSize::Qword)
}

#[test]
fn vpord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 235, 253], OperandSize::Dword)
}

#[test]
fn vpord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 203, 235, 1], OperandSize::Dword)
}

#[test]
fn vpord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 386332747, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 221, 235, 184, 75, 248, 6, 23], OperandSize::Dword)
}

#[test]
fn vpord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 109, 193, 235, 200], OperandSize::Qword)
}

#[test]
fn vpord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 266178413, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 101, 204, 235, 52, 253, 109, 143, 221, 15], OperandSize::Qword)
}

#[test]
fn vpord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 13, 213, 235, 4, 136], OperandSize::Qword)
}

