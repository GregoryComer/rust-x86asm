use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn valignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 85, 143, 3, 204, 64], OperandSize::Dword)
}

fn valignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 606774021, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 139, 3, 140, 122, 5, 163, 42, 36, 75], OperandSize::Dword)
}

fn valignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 808902260, Some(OperandSize::Dword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 85, 154, 3, 188, 82, 116, 222, 54, 48, 17], OperandSize::Dword)
}

fn valignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 77, 131, 3, 210, 77], OperandSize::Qword)
}

fn valignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 77, 139, 3, 59, 95], OperandSize::Qword)
}

fn valignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 733173239, Some(OperandSize::Dword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 93, 147, 3, 172, 250, 247, 85, 179, 43, 2], OperandSize::Qword)
}

fn valignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 93, 171, 3, 249, 78], OperandSize::Dword)
}

fn valignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 172, 3, 20, 90, 22], OperandSize::Dword)
}

fn valignd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 760111660, Some(OperandSize::Dword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 93, 185, 3, 185, 44, 98, 78, 45, 92], OperandSize::Dword)
}

fn valignd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 195, 101, 166, 3, 238, 31], OperandSize::Qword)
}

fn valignd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RCX, 719083454, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 5, 167, 3, 153, 190, 87, 220, 42, 26], OperandSize::Qword)
}

fn valignd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RSI, 121873657, Some(OperandSize::Dword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 109, 181, 3, 182, 249, 164, 67, 7, 10], OperandSize::Qword)
}

fn valignd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 204, 3, 211, 45], OperandSize::Dword)
}

fn valignd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 206, 3, 18, 27], OperandSize::Dword)
}

fn valignd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 410015374, Some(OperandSize::Dword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 101, 221, 3, 172, 151, 142, 86, 112, 24, 85], OperandSize::Dword)
}

fn valignd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM10)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 67, 61, 199, 3, 218, 101], OperandSize::Qword)
}

fn valignd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 45, 195, 3, 4, 187, 114], OperandSize::Qword)
}

fn valignd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 2093133841, Some(OperandSize::Dword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 37, 223, 3, 156, 240, 17, 176, 194, 124, 124], OperandSize::Qword)
}

