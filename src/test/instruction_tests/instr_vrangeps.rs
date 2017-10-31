use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangeps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 69, 143, 80, 195, 84], OperandSize::Dword)
}

#[test]
fn vrangeps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 139, 80, 44, 118, 3], OperandSize::Dword)
}

#[test]
fn vrangeps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 154, 80, 43, 22], OperandSize::Dword)
}

#[test]
fn vrangeps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 51, 77, 143, 80, 234, 97], OperandSize::Qword)
}

#[test]
fn vrangeps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RAX, 1265558262, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 37, 134, 80, 168, 246, 226, 110, 75, 75], OperandSize::Qword)
}

#[test]
fn vrangeps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1859196751, Some(OperandSize::Dword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 69, 157, 80, 60, 213, 79, 23, 209, 110, 101], OperandSize::Qword)
}

#[test]
fn vrangeps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 172, 80, 244, 126], OperandSize::Dword)
}

#[test]
fn vrangeps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 69, 175, 80, 44, 115, 12], OperandSize::Dword)
}

#[test]
fn vrangeps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 374529809, Some(OperandSize::Dword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 117, 191, 80, 138, 17, 223, 82, 22, 37], OperandSize::Dword)
}

#[test]
fn vrangeps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM25)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 19, 109, 174, 80, 225, 58], OperandSize::Qword)
}

#[test]
fn vrangeps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1658821970, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 45, 166, 80, 20, 181, 82, 157, 223, 98, 117], OperandSize::Qword)
}

#[test]
fn vrangeps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 919635807, Some(OperandSize::Dword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 21, 177, 80, 140, 194, 95, 135, 208, 54, 86], OperandSize::Qword)
}

#[test]
fn vrangeps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 159, 80, 217, 71], OperandSize::Dword)
}

#[test]
fn vrangeps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2070949430, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 201, 80, 12, 205, 54, 46, 112, 123, 23], OperandSize::Dword)
}

#[test]
fn vrangeps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 773788227, Some(OperandSize::Dword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 85, 218, 80, 12, 181, 67, 18, 31, 46, 119], OperandSize::Dword)
}

#[test]
fn vrangeps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 61, 151, 80, 218, 55], OperandSize::Qword)
}

#[test]
fn vrangeps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RSI, 817231744, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 5, 193, 80, 142, 128, 247, 181, 48, 86], OperandSize::Qword)
}

#[test]
fn vrangeps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 109, 212, 80, 43, 25], OperandSize::Qword)
}

