use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 253, 249], OperandSize::Dword)
}

#[test]
fn vpaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 253, 60, 191], OperandSize::Dword)
}

#[test]
fn vpaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 253, 206], OperandSize::Qword)
}

#[test]
fn vpaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2069744600, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 253, 20, 205, 216, 203, 93, 123], OperandSize::Qword)
}

#[test]
fn vpaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 253, 209], OperandSize::Dword)
}

#[test]
fn vpaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 253, 33], OperandSize::Dword)
}

#[test]
fn vpaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 253, 205], OperandSize::Qword)
}

#[test]
fn vpaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 253, 4, 130], OperandSize::Qword)
}

#[test]
fn vpaddw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 253, 244], OperandSize::Dword)
}

#[test]
fn vpaddw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 253, 27], OperandSize::Dword)
}

#[test]
fn vpaddw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 5, 131, 253, 223], OperandSize::Qword)
}

#[test]
fn vpaddw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1038870638, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 93, 142, 253, 44, 197, 110, 232, 235, 61], OperandSize::Qword)
}

#[test]
fn vpaddw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 253, 201], OperandSize::Dword)
}

#[test]
fn vpaddw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 413817313, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 253, 134, 225, 89, 170, 24], OperandSize::Dword)
}

#[test]
fn vpaddw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 101, 163, 253, 194], OperandSize::Qword)
}

#[test]
fn vpaddw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RDX, 1835079963, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 101, 173, 253, 130, 27, 25, 97, 109], OperandSize::Qword)
}

