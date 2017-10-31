use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 33, 222], OperandSize::Dword)
}

#[test]
fn vpmovsdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(EDI, 1674912878, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 33, 167, 110, 36, 213, 99], OperandSize::Dword)
}

#[test]
fn vpmovsdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 126, 143, 33, 251], OperandSize::Qword)
}

#[test]
fn vpmovsdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 33, 4, 75], OperandSize::Qword)
}

#[test]
fn vpmovsdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 33, 246], OperandSize::Dword)
}

#[test]
fn vpmovsdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 33, 51], OperandSize::Dword)
}

#[test]
fn vpmovsdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM23)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 126, 170, 33, 239], OperandSize::Qword)
}

#[test]
fn vpmovsdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(RCX, 1149985686, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 33, 153, 150, 99, 139, 68], OperandSize::Qword)
}

#[test]
fn vpmovsdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 33, 223], OperandSize::Dword)
}

#[test]
fn vpmovsdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(EDX, 1161969010, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 33, 154, 114, 61, 66, 69], OperandSize::Dword)
}

#[test]
fn vpmovsdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 203, 33, 197], OperandSize::Qword)
}

#[test]
fn vpmovsdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 33, 46], OperandSize::Qword)
}

