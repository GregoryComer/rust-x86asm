use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 223, 224], OperandSize::Dword)
}

#[test]
fn vpandnd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 255797054, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 137, 223, 36, 253, 62, 39, 63, 15], OperandSize::Dword)
}

#[test]
fn vpandnd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 858604137, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 158, 223, 152, 105, 66, 45, 51], OperandSize::Dword)
}

#[test]
fn vpandnd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 109, 143, 223, 248], OperandSize::Qword)
}

#[test]
fn vpandnd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RSI, 981275139, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 77, 131, 223, 166, 3, 18, 125, 58], OperandSize::Qword)
}

#[test]
fn vpandnd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RDI, 1757515951, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 69, 151, 223, 175, 175, 144, 193, 104], OperandSize::Qword)
}

#[test]
fn vpandnd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 223, 210], OperandSize::Dword)
}

#[test]
fn vpandnd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 2133134860, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 223, 161, 12, 14, 37, 127], OperandSize::Dword)
}

#[test]
fn vpandnd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 2957005, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 77, 190, 223, 161, 205, 30, 45, 0], OperandSize::Dword)
}

#[test]
fn vpandnd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 37, 162, 223, 224], OperandSize::Qword)
}

#[test]
fn vpandnd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RCX, 1646848709, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 53, 173, 223, 137, 197, 234, 40, 98], OperandSize::Qword)
}

#[test]
fn vpandnd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RCX, 283621362, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 61, 179, 223, 161, 242, 183, 231, 16], OperandSize::Qword)
}

#[test]
fn vpandnd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 223, 202], OperandSize::Dword)
}

#[test]
fn vpandnd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 841104936, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 223, 20, 85, 40, 62, 34, 50], OperandSize::Dword)
}

#[test]
fn vpandnd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1371240727, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 217, 223, 36, 117, 23, 121, 187, 81], OperandSize::Dword)
}

#[test]
fn vpandnd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 29, 198, 223, 214], OperandSize::Qword)
}

#[test]
fn vpandnd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 116541425, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 101, 201, 223, 164, 145, 241, 71, 242, 6], OperandSize::Qword)
}

#[test]
fn vpandnd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1983530345, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 210, 223, 180, 147, 105, 69, 58, 118], OperandSize::Qword)
}

