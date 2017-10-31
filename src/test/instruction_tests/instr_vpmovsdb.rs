use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 33, 245], OperandSize::Dword)
}

#[test]
fn vpmovsdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 33, 12, 90], OperandSize::Dword)
}

#[test]
fn vpmovsdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 126, 141, 33, 210], OperandSize::Qword)
}

#[test]
fn vpmovsdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(RSI, 1401952076, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 33, 182, 76, 23, 144, 83], OperandSize::Qword)
}

#[test]
fn vpmovsdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 33, 219], OperandSize::Dword)
}

#[test]
fn vpmovsdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1028779779, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 33, 12, 253, 3, 239, 81, 61], OperandSize::Dword)
}

#[test]
fn vpmovsdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 126, 174, 33, 204], OperandSize::Qword)
}

#[test]
fn vpmovsdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledDisplaced(RCX, Four, 2132270347, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 33, 20, 141, 11, 221, 23, 127], OperandSize::Qword)
}

#[test]
fn vpmovsdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 33, 224], OperandSize::Dword)
}

#[test]
fn vpmovsdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 33, 15], OperandSize::Dword)
}

#[test]
fn vpmovsdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 126, 202, 33, 232], OperandSize::Qword)
}

#[test]
fn vpmovsdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1415716027, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 33, 180, 142, 187, 28, 98, 84], OperandSize::Qword)
}

