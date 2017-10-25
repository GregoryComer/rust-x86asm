use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreduceps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 86, 223, 5], OperandSize::Dword)
}

#[test]
fn vreduceps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1553466172, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 86, 28, 117, 60, 3, 152, 92, 7], OperandSize::Dword)
}

#[test]
fn vreduceps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 154, 86, 0, 54], OperandSize::Dword)
}

#[test]
fn vreduceps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM18)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 125, 141, 86, 226, 49], OperandSize::Qword)
}

#[test]
fn vreduceps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 924949071, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 125, 139, 86, 44, 245, 79, 154, 33, 55, 12], OperandSize::Qword)
}

#[test]
fn vreduceps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(XMM15)), operand2: Some(IndirectDisplaced(RAX, 675493458, Some(OperandSize::Dword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 125, 153, 86, 184, 82, 54, 67, 40, 81], OperandSize::Qword)
}

#[test]
fn vreduceps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 86, 224, 31], OperandSize::Dword)
}

#[test]
fn vreduceps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 86, 36, 202, 22], OperandSize::Dword)
}

#[test]
fn vreduceps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1318935127, Some(OperandSize::Dword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 189, 86, 4, 149, 87, 90, 157, 78, 104], OperandSize::Dword)
}

#[test]
fn vreduceps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM8)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 125, 171, 86, 224, 82], OperandSize::Qword)
}

#[test]
fn vreduceps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM23)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 125, 173, 86, 63, 126], OperandSize::Qword)
}

#[test]
fn vreduceps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 125, 191, 86, 36, 64, 72], OperandSize::Qword)
}

#[test]
fn vreduceps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 157, 86, 197, 31], OperandSize::Dword)
}

#[test]
fn vreduceps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(EBX, 1980093803, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 201, 86, 155, 107, 213, 5, 118, 54], OperandSize::Dword)
}

#[test]
fn vreduceps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EAX, 2017503543, Some(OperandSize::Dword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 221, 86, 136, 55, 169, 64, 120, 41], OperandSize::Dword)
}

#[test]
fn vreduceps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM23)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 35, 125, 156, 86, 223, 120], OperandSize::Qword)
}

#[test]
fn vreduceps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 125, 205, 86, 28, 79, 97], OperandSize::Qword)
}

#[test]
fn vreduceps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPS, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 217, 86, 12, 254, 5], OperandSize::Qword)
}

