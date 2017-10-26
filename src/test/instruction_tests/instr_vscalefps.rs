use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 44, 194], OperandSize::Dword)
}

#[test]
fn vscalefps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 44, 60, 190], OperandSize::Dword)
}

#[test]
fn vscalefps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1433984840, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 155, 44, 12, 181, 72, 223, 120, 85], OperandSize::Dword)
}

#[test]
fn vscalefps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 53, 142, 44, 241], OperandSize::Qword)
}

#[test]
fn vscalefps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1508233935, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 13, 137, 44, 180, 246, 207, 210, 229, 89], OperandSize::Qword)
}

#[test]
fn vscalefps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 677706488, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 156, 44, 179, 248, 250, 100, 40], OperandSize::Qword)
}

#[test]
fn vscalefps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 44, 241], OperandSize::Dword)
}

#[test]
fn vscalefps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1106635818, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 44, 148, 218, 42, 236, 245, 65], OperandSize::Dword)
}

#[test]
fn vscalefps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1630277463, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 186, 44, 140, 208, 87, 15, 44, 97], OperandSize::Dword)
}

#[test]
fn vscalefps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 53, 164, 44, 203], OperandSize::Qword)
}

#[test]
fn vscalefps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 44, 2], OperandSize::Qword)
}

#[test]
fn vscalefps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 13, 181, 44, 44, 130], OperandSize::Qword)
}

#[test]
fn vscalefps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 249, 44, 240], OperandSize::Dword)
}

#[test]
fn vscalefps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 205, 44, 58], OperandSize::Dword)
}

#[test]
fn vscalefps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 218, 44, 60, 78], OperandSize::Dword)
}

#[test]
fn vscalefps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 125, 215, 44, 207], OperandSize::Qword)
}

#[test]
fn vscalefps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1693312155, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 69, 204, 44, 180, 135, 155, 228, 237, 100], OperandSize::Qword)
}

#[test]
fn vscalefps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 109, 210, 44, 60, 183], OperandSize::Qword)
}

