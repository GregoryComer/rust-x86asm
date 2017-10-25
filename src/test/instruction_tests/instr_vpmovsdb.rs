use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 33, 198], OperandSize::Dword)
}

#[test]
fn vpmovsdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(EDI, 1969416890, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 33, 183, 186, 234, 98, 117], OperandSize::Dword)
}

#[test]
fn vpmovsdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 126, 140, 33, 236], OperandSize::Qword)
}

#[test]
fn vpmovsdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1398973256, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 33, 148, 113, 72, 163, 98, 83], OperandSize::Qword)
}

#[test]
fn vpmovsdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 33, 237], OperandSize::Dword)
}

#[test]
fn vpmovsdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledDisplaced(EDX, Four, 87076138, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 33, 12, 149, 42, 173, 48, 5], OperandSize::Dword)
}

#[test]
fn vpmovsdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 126, 170, 33, 233], OperandSize::Qword)
}

#[test]
fn vpmovsdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 33, 46], OperandSize::Qword)
}

#[test]
fn vpmovsdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 33, 195], OperandSize::Dword)
}

#[test]
fn vpmovsdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 33, 28, 154], OperandSize::Dword)
}

#[test]
fn vpmovsdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 33, 252], OperandSize::Qword)
}

#[test]
fn vpmovsdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 33, 34], OperandSize::Qword)
}

