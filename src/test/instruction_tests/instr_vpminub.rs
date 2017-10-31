use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 218, 251], OperandSize::Dword)
}

#[test]
fn vpminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1425817399, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 218, 20, 149, 55, 63, 252, 84], OperandSize::Dword)
}

#[test]
fn vpminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 218, 204], OperandSize::Qword)
}

#[test]
fn vpminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 811507392, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 218, 148, 70, 192, 158, 94, 48], OperandSize::Qword)
}

#[test]
fn vpminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 218, 216], OperandSize::Dword)
}

#[test]
fn vpminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1770956034, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 218, 28, 213, 2, 165, 142, 105], OperandSize::Dword)
}

#[test]
fn vpminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 218, 216], OperandSize::Qword)
}

#[test]
fn vpminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 206186259, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 218, 172, 250, 19, 39, 74, 12], OperandSize::Qword)
}

#[test]
fn vpminub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 218, 238], OperandSize::Dword)
}

#[test]
fn vpminub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 882019670, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 218, 140, 154, 86, 141, 146, 52], OperandSize::Dword)
}

#[test]
fn vpminub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 85, 138, 218, 210], OperandSize::Qword)
}

#[test]
fn vpminub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 1924460326, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 69, 143, 218, 139, 38, 239, 180, 114], OperandSize::Qword)
}

#[test]
fn vpminub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 218, 244], OperandSize::Dword)
}

#[test]
fn vpminub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 948508066, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 218, 28, 85, 162, 21, 137, 56], OperandSize::Dword)
}

#[test]
fn vpminub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 169, 218, 212], OperandSize::Qword)
}

#[test]
fn vpminub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM11)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 37, 175, 218, 17], OperandSize::Qword)
}

#[test]
fn vpminub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 218, 223], OperandSize::Dword)
}

#[test]
fn vpminub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 1483100467, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 206, 218, 180, 210, 51, 81, 102, 88], OperandSize::Dword)
}

#[test]
fn vpminub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 77, 201, 218, 195], OperandSize::Qword)
}

#[test]
fn vpminub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1451006701, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 53, 201, 218, 4, 93, 237, 154, 124, 86], OperandSize::Qword)
}

