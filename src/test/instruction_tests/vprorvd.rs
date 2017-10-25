use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprorvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 20, 250], OperandSize::Dword)
}

fn vprorvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 455737172, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 20, 178, 84, 255, 41, 27], OperandSize::Dword)
}

fn vprorvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 159, 20, 41], OperandSize::Dword)
}

fn vprorvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 61, 141, 20, 246], OperandSize::Qword)
}

fn vprorvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 448569840, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 109, 137, 20, 172, 201, 240, 161, 188, 26], OperandSize::Qword)
}

fn vprorvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 61, 153, 20, 46], OperandSize::Qword)
}

fn vprorvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 20, 223], OperandSize::Dword)
}

fn vprorvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 20, 12, 158], OperandSize::Dword)
}

fn vprorvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 1810496113, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 186, 20, 171, 113, 250, 233, 107], OperandSize::Dword)
}

fn vprorvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 21, 165, 20, 194], OperandSize::Qword)
}

fn vprorvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectDisplaced(RSI, 1506199137, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 45, 173, 20, 158, 97, 198, 198, 89], OperandSize::Qword)
}

fn vprorvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 29, 189, 20, 8], OperandSize::Qword)
}

fn vprorvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 20, 219], OperandSize::Dword)
}

fn vprorvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EDI, 354522426, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 20, 167, 58, 149, 33, 21], OperandSize::Dword)
}

fn vprorvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 920512448, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 20, 140, 83, 192, 231, 221, 54], OperandSize::Dword)
}

fn vprorvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 93, 205, 20, 195], OperandSize::Qword)
}

fn vprorvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RSI, 112456732, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 37, 193, 20, 174, 28, 244, 179, 6], OperandSize::Qword)
}

fn vprorvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 5, 214, 20, 59], OperandSize::Qword)
}

