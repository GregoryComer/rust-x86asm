use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 13, 30, 204, 122], OperandSize::Dword)
}

fn vpcmpud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 677651491, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 93, 15, 30, 28, 85, 35, 36, 100, 40, 119], OperandSize::Dword)
}

fn vpcmpud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1420915123, Some(OperandSize::Dword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 28, 30, 163, 179, 113, 177, 84, 3], OperandSize::Dword)
}

fn vpcmpud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 117, 6, 30, 225, 3], OperandSize::Qword)
}

fn vpcmpud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1812454924, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 4, 30, 36, 77, 12, 222, 7, 108, 124], OperandSize::Qword)
}

fn vpcmpud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 53, 19, 30, 20, 185, 18], OperandSize::Qword)
}

fn vpcmpud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 44, 30, 209, 59], OperandSize::Dword)
}

fn vpcmpud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1708589350, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 44, 30, 20, 157, 38, 1, 215, 101, 71], OperandSize::Dword)
}

fn vpcmpud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 2042445541, Some(OperandSize::Dword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 117, 61, 30, 12, 221, 229, 62, 189, 121, 103], OperandSize::Dword)
}

fn vpcmpud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM31)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 125, 45, 30, 247, 95], OperandSize::Qword)
}

fn vpcmpud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 42, 30, 32, 62], OperandSize::Qword)
}

fn vpcmpud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 551196541, Some(OperandSize::Dword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 13, 61, 30, 180, 74, 125, 151, 218, 32, 101], OperandSize::Qword)
}

fn vpcmpud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 77, 30, 250, 95], OperandSize::Dword)
}

fn vpcmpud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 79, 30, 14, 97], OperandSize::Dword)
}

fn vpcmpud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 117, 94, 30, 43, 119], OperandSize::Dword)
}

fn vpcmpud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM10)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 5, 74, 30, 202, 5], OperandSize::Qword)
}

fn vpcmpud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 37, 79, 30, 20, 210, 91], OperandSize::Qword)
}

fn vpcmpud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 61, 93, 30, 40, 97], OperandSize::Qword)
}

