use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 54, 224], OperandSize::Dword)
}

#[test]
fn vpermd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 196410424, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 54, 4, 125, 56, 252, 180, 11], OperandSize::Dword)
}

#[test]
fn vpermd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 54, 197], OperandSize::Qword)
}

#[test]
fn vpermd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 54, 20, 192], OperandSize::Qword)
}

#[test]
fn vpermd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 54, 240], OperandSize::Dword)
}

#[test]
fn vpermd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 54, 27], OperandSize::Dword)
}

#[test]
fn vpermd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 185, 54, 1], OperandSize::Dword)
}

#[test]
fn vpermd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 161, 54, 248], OperandSize::Qword)
}

#[test]
fn vpermd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM23)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 69, 163, 54, 14], OperandSize::Qword)
}

#[test]
fn vpermd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 117, 179, 54, 2], OperandSize::Qword)
}

#[test]
fn vpermd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 54, 230], OperandSize::Dword)
}

#[test]
fn vpermd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 340450253, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 54, 164, 219, 205, 219, 74, 20], OperandSize::Dword)
}

#[test]
fn vpermd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 220, 54, 12, 126], OperandSize::Dword)
}

#[test]
fn vpermd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 101, 193, 54, 200], OperandSize::Qword)
}

#[test]
fn vpermd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 101, 205, 54, 28, 177], OperandSize::Qword)
}

#[test]
fn vpermd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 219092868, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 220, 54, 4, 141, 132, 23, 15, 13], OperandSize::Qword)
}

