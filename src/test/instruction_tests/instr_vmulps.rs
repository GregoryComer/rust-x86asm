use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 89, 250], OperandSize::Dword)
}

#[test]
fn vmulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 736705275, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 89, 134, 251, 58, 233, 43], OperandSize::Dword)
}

#[test]
fn vmulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 89, 249], OperandSize::Qword)
}

#[test]
fn vmulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 89, 31], OperandSize::Qword)
}

#[test]
fn vmulps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 89, 254], OperandSize::Dword)
}

#[test]
fn vmulps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 274463284, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 89, 156, 254, 52, 250, 91, 16], OperandSize::Dword)
}

#[test]
fn vmulps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 89, 234], OperandSize::Qword)
}

#[test]
fn vmulps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 693114996, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 89, 44, 157, 116, 24, 80, 41], OperandSize::Qword)
}

#[test]
fn vmulps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 116, 137, 89, 244], OperandSize::Dword)
}

#[test]
fn vmulps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 141, 89, 40], OperandSize::Dword)
}

#[test]
fn vmulps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 451241699, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 154, 89, 36, 213, 227, 102, 229, 26], OperandSize::Dword)
}

#[test]
fn vmulps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 52, 137, 89, 203], OperandSize::Qword)
}

#[test]
fn vmulps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RDI, 1071140824, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 20, 131, 89, 159, 216, 79, 216, 63], OperandSize::Qword)
}

#[test]
fn vmulps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 68, 149, 89, 25], OperandSize::Qword)
}

#[test]
fn vmulps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 169, 89, 248], OperandSize::Dword)
}

#[test]
fn vmulps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1780265020, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 173, 89, 44, 253, 60, 176, 28, 106], OperandSize::Dword)
}

#[test]
fn vmulps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 185, 89, 60, 113], OperandSize::Dword)
}

#[test]
fn vmulps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 84, 166, 89, 209], OperandSize::Qword)
}

#[test]
fn vmulps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1313188278, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 170, 89, 36, 205, 182, 169, 69, 78], OperandSize::Qword)
}

#[test]
fn vmulps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 467550957, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 116, 191, 89, 20, 93, 237, 66, 222, 27], OperandSize::Qword)
}

#[test]
fn vmulps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 68, 252, 89, 202], OperandSize::Dword)
}

#[test]
fn vmulps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 202, 89, 22], OperandSize::Dword)
}

#[test]
fn vmulps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 220, 89, 36, 201], OperandSize::Dword)
}

#[test]
fn vmulps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 100, 156, 89, 243], OperandSize::Qword)
}

#[test]
fn vmulps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2063656287, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 4, 203, 89, 28, 93, 95, 229, 0, 123], OperandSize::Qword)
}

#[test]
fn vmulps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 110893383, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 28, 214, 89, 164, 135, 71, 25, 156, 6], OperandSize::Qword)
}

