use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 142, 235, 204], OperandSize::Dword)
}

#[test]
fn vpord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 228479337, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 137, 235, 188, 202, 105, 81, 158, 13], OperandSize::Dword)
}

#[test]
fn vpord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 153, 235, 12, 130], OperandSize::Dword)
}

#[test]
fn vpord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 235, 203], OperandSize::Qword)
}

#[test]
fn vpord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RAX, 350716207, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 143, 235, 144, 47, 129, 231, 20], OperandSize::Qword)
}

#[test]
fn vpord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RAX, 2145502222, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 45, 157, 235, 128, 14, 196, 225, 127], OperandSize::Qword)
}

#[test]
fn vpord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 235, 240], OperandSize::Dword)
}

#[test]
fn vpord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 651459893, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 235, 52, 149, 53, 125, 212, 38], OperandSize::Dword)
}

#[test]
fn vpord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 77, 187, 235, 34], OperandSize::Dword)
}

#[test]
fn vpord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 77, 171, 235, 254], OperandSize::Qword)
}

#[test]
fn vpord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1800422621, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 5, 164, 235, 36, 197, 221, 68, 80, 107], OperandSize::Qword)
}

#[test]
fn vpord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RCX, 1666584384, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 61, 177, 235, 185, 64, 15, 86, 99], OperandSize::Qword)
}

#[test]
fn vpord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 202, 235, 241], OperandSize::Dword)
}

#[test]
fn vpord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 1688742032, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 235, 145, 144, 40, 168, 100], OperandSize::Dword)
}

#[test]
fn vpord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDI, 381309863, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 220, 235, 159, 167, 83, 186, 22], OperandSize::Dword)
}

#[test]
fn vpord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 13, 201, 235, 234], OperandSize::Qword)
}

#[test]
fn vpord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 195, 235, 55], OperandSize::Qword)
}

#[test]
fn vpord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RDX, 74398827, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 222, 235, 130, 107, 60, 111, 4], OperandSize::Qword)
}

