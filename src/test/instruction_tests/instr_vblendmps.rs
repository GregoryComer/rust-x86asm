use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 101, 222], OperandSize::Dword)
}

#[test]
fn vblendmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1516065615, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 139, 101, 12, 149, 79, 83, 93, 90], OperandSize::Dword)
}

#[test]
fn vblendmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 159, 101, 23], OperandSize::Dword)
}

#[test]
fn vblendmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 53, 133, 101, 200], OperandSize::Qword)
}

#[test]
fn vblendmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RCX, 442482238, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 130, 101, 185, 62, 190, 95, 26], OperandSize::Qword)
}

#[test]
fn vblendmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 427429688, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 69, 156, 101, 12, 117, 56, 15, 122, 25], OperandSize::Qword)
}

#[test]
fn vblendmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 101, 250], OperandSize::Dword)
}

#[test]
fn vblendmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 414036283, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 101, 180, 199, 59, 177, 173, 24], OperandSize::Dword)
}

#[test]
fn vblendmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 2079744004, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 101, 28, 69, 4, 96, 246, 123], OperandSize::Dword)
}

#[test]
fn vblendmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 61, 174, 101, 250], OperandSize::Qword)
}

#[test]
fn vblendmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RSI, 2138120020, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 93, 170, 101, 142, 84, 31, 113, 127], OperandSize::Qword)
}

#[test]
fn vblendmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1215260989, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 13, 177, 101, 164, 199, 61, 105, 111, 72], OperandSize::Qword)
}

#[test]
fn vblendmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 202, 101, 201], OperandSize::Dword)
}

#[test]
fn vblendmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 185186111, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 101, 36, 117, 63, 183, 9, 11], OperandSize::Dword)
}

#[test]
fn vblendmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1970930106, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 101, 132, 65, 186, 1, 122, 117], OperandSize::Dword)
}

#[test]
fn vblendmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 93, 202, 101, 194], OperandSize::Qword)
}

#[test]
fn vblendmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 29, 196, 101, 60, 243], OperandSize::Qword)
}

#[test]
fn vblendmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RDX, 1125487436, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 53, 221, 101, 130, 76, 147, 21, 67], OperandSize::Qword)
}

