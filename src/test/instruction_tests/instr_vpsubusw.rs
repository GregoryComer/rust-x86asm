use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 217, 244], OperandSize::Dword)
}

#[test]
fn vpsubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 844677161, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 217, 52, 253, 41, 192, 88, 50], OperandSize::Dword)
}

#[test]
fn vpsubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 217, 249], OperandSize::Qword)
}

#[test]
fn vpsubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 217, 41], OperandSize::Qword)
}

#[test]
fn vpsubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 217, 213], OperandSize::Dword)
}

#[test]
fn vpsubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 561276535, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 217, 188, 158, 119, 102, 116, 33], OperandSize::Dword)
}

#[test]
fn vpsubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 217, 205], OperandSize::Qword)
}

#[test]
fn vpsubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 736000466, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 217, 28, 125, 210, 121, 222, 43], OperandSize::Qword)
}

#[test]
fn vpsubusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 217, 220], OperandSize::Dword)
}

#[test]
fn vpsubusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 2056309837, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 217, 131, 77, 204, 144, 122], OperandSize::Dword)
}

#[test]
fn vpsubusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 37, 141, 217, 227], OperandSize::Qword)
}

#[test]
fn vpsubusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 69, 138, 217, 20, 87], OperandSize::Qword)
}

#[test]
fn vpsubusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 171, 217, 192], OperandSize::Dword)
}

#[test]
fn vpsubusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1515631099, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 173, 217, 20, 85, 251, 177, 86, 90], OperandSize::Dword)
}

#[test]
fn vpsubusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 5, 167, 217, 245], OperandSize::Qword)
}

#[test]
fn vpsubusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 61, 175, 217, 20, 138], OperandSize::Qword)
}

