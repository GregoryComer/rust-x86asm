use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 44, 254], OperandSize::Dword)
}

#[test]
fn vscalefps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 940938842, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 44, 185, 90, 150, 21, 56], OperandSize::Dword)
}

#[test]
fn vscalefps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 156, 44, 4, 217], OperandSize::Dword)
}

#[test]
fn vscalefps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 117, 133, 44, 231], OperandSize::Qword)
}

#[test]
fn vscalefps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RSI, 1761466259, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 5, 137, 44, 190, 147, 215, 253, 104], OperandSize::Qword)
}

#[test]
fn vscalefps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 53, 145, 44, 20, 131], OperandSize::Qword)
}

#[test]
fn vscalefps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 44, 254], OperandSize::Dword)
}

#[test]
fn vscalefps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 169, 44, 58], OperandSize::Dword)
}

#[test]
fn vscalefps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 370797009, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 189, 44, 28, 85, 209, 233, 25, 22], OperandSize::Dword)
}

#[test]
fn vscalefps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 109, 161, 44, 221], OperandSize::Qword)
}

#[test]
fn vscalefps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 219984226, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 21, 174, 44, 44, 69, 98, 177, 28, 13], OperandSize::Qword)
}

#[test]
fn vscalefps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RDX, 1303337943, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 109, 179, 44, 154, 215, 91, 175, 77], OperandSize::Qword)
}

#[test]
fn vscalefps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 252, 44, 249], OperandSize::Dword)
}

#[test]
fn vscalefps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 526071728, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 44, 170, 176, 55, 91, 31], OperandSize::Dword)
}

#[test]
fn vscalefps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 823080518, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 217, 44, 44, 69, 70, 54, 15, 49], OperandSize::Dword)
}

#[test]
fn vscalefps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 61, 155, 44, 245], OperandSize::Qword)
}

#[test]
fn vscalefps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 11619524, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 21, 198, 44, 12, 189, 196, 76, 177, 0], OperandSize::Qword)
}

#[test]
fn vscalefps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1448396809, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 21, 218, 44, 180, 214, 9, 200, 84, 86], OperandSize::Qword)
}

