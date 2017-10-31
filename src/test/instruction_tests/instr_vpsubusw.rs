use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 217, 199], OperandSize::Dword)
}

#[test]
fn vpsubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1622033599, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 217, 171, 191, 68, 174, 96], OperandSize::Dword)
}

#[test]
fn vpsubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 217, 211], OperandSize::Qword)
}

#[test]
fn vpsubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 217, 27], OperandSize::Qword)
}

#[test]
fn vpsubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 217, 253], OperandSize::Dword)
}

#[test]
fn vpsubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 217, 48], OperandSize::Dword)
}

#[test]
fn vpsubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 217, 195], OperandSize::Qword)
}

#[test]
fn vpsubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 325913610, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 217, 60, 149, 10, 12, 109, 19], OperandSize::Qword)
}

#[test]
fn vpsubusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 217, 243], OperandSize::Dword)
}

#[test]
fn vpsubusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 636600843, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 217, 139, 11, 194, 241, 37], OperandSize::Dword)
}

#[test]
fn vpsubusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 93, 132, 217, 209], OperandSize::Qword)
}

#[test]
fn vpsubusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 85, 135, 217, 12, 144], OperandSize::Qword)
}

#[test]
fn vpsubusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 217, 250], OperandSize::Dword)
}

#[test]
fn vpsubusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 217, 47], OperandSize::Dword)
}

#[test]
fn vpsubusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 21, 161, 217, 209], OperandSize::Qword)
}

#[test]
fn vpsubusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 685932004, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 85, 169, 217, 188, 202, 228, 125, 226, 40], OperandSize::Qword)
}

