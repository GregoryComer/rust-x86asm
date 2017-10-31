use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 66, 252], OperandSize::Dword)
}

#[test]
fn vgetexpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 343125189, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 66, 156, 179, 197, 172, 115, 20], OperandSize::Dword)
}

#[test]
fn vgetexpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1414694650, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 66, 180, 202, 250, 134, 82, 84], OperandSize::Dword)
}

#[test]
fn vgetexpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 138, 66, 254], OperandSize::Qword)
}

#[test]
fn vgetexpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 142, 66, 12, 130], OperandSize::Qword)
}

#[test]
fn vgetexpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 784577119, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 66, 156, 187, 95, 178, 195, 46], OperandSize::Qword)
}

#[test]
fn vgetexpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 66, 212], OperandSize::Dword)
}

#[test]
fn vgetexpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDX, 1697469956, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 66, 186, 4, 86, 45, 101], OperandSize::Dword)
}

#[test]
fn vgetexpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 338568271, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 66, 188, 113, 79, 36, 46, 20], OperandSize::Dword)
}

#[test]
fn vgetexpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 66, 223], OperandSize::Qword)
}

#[test]
fn vgetexpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM27)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 174, 66, 24], OperandSize::Qword)
}

#[test]
fn vgetexpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 103464134, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 185, 66, 164, 136, 198, 188, 42, 6], OperandSize::Qword)
}

#[test]
fn vgetexpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 155, 66, 249], OperandSize::Dword)
}

#[test]
fn vgetexpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 66, 47], OperandSize::Dword)
}

#[test]
fn vgetexpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 99497977, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 66, 132, 199, 249, 55, 238, 5], OperandSize::Dword)
}

#[test]
fn vgetexpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 125, 154, 66, 219], OperandSize::Qword)
}

#[test]
fn vgetexpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 2087154920, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 206, 66, 20, 253, 232, 116, 103, 124], OperandSize::Qword)
}

#[test]
fn vgetexpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM24)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 218, 66, 6], OperandSize::Qword)
}

