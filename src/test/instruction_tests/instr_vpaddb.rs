use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 252, 220], OperandSize::Dword)
}

#[test]
fn vpaddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 252, 41], OperandSize::Dword)
}

#[test]
fn vpaddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 252, 208], OperandSize::Qword)
}

#[test]
fn vpaddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 2105175818, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 252, 36, 213, 10, 111, 122, 125], OperandSize::Qword)
}

#[test]
fn vpaddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 252, 203], OperandSize::Dword)
}

#[test]
fn vpaddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 252, 12, 82], OperandSize::Dword)
}

#[test]
fn vpaddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 252, 239], OperandSize::Qword)
}

#[test]
fn vpaddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 2067507484, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 252, 156, 118, 28, 169, 59, 123], OperandSize::Qword)
}

#[test]
fn vpaddb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 252, 252], OperandSize::Dword)
}

#[test]
fn vpaddb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 252, 58], OperandSize::Dword)
}

#[test]
fn vpaddb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 93, 141, 252, 206], OperandSize::Qword)
}

#[test]
fn vpaddb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1306195935, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 129, 252, 164, 219, 223, 247, 218, 77], OperandSize::Qword)
}

#[test]
fn vpaddb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 252, 239], OperandSize::Dword)
}

#[test]
fn vpaddb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 972312010, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 252, 179, 202, 77, 244, 57], OperandSize::Dword)
}

#[test]
fn vpaddb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 29, 161, 252, 195], OperandSize::Qword)
}

#[test]
fn vpaddb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 343534051, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 13, 172, 252, 180, 90, 227, 233, 121, 20], OperandSize::Qword)
}

