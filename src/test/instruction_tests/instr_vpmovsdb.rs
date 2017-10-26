use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 33, 224], OperandSize::Dword)
}

#[test]
fn vpmovsdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(EDI, 710716101, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 33, 175, 197, 170, 92, 42], OperandSize::Dword)
}

#[test]
fn vpmovsdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 126, 139, 33, 255], OperandSize::Qword)
}

#[test]
fn vpmovsdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 33, 46], OperandSize::Qword)
}

#[test]
fn vpmovsdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 33, 192], OperandSize::Dword)
}

#[test]
fn vpmovsdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 33, 41], OperandSize::Dword)
}

#[test]
fn vpmovsdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM14)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 126, 169, 33, 254], OperandSize::Qword)
}

#[test]
fn vpmovsdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 33, 63], OperandSize::Qword)
}

#[test]
fn vpmovsdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 33, 247], OperandSize::Dword)
}

#[test]
fn vpmovsdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(EAX, 1077889086, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 33, 136, 62, 72, 63, 64], OperandSize::Dword)
}

#[test]
fn vpmovsdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 126, 207, 33, 244], OperandSize::Qword)
}

#[test]
fn vpmovsdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1901108148, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 33, 172, 87, 180, 155, 80, 113], OperandSize::Qword)
}

