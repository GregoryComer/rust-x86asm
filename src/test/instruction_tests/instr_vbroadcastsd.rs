use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 39], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RSI, 537144304, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 182, 240, 43, 4, 32], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 242], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 207], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 25, 232], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1699042323, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 25, 156, 127, 19, 84, 69, 101], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 253, 169, 25, 201], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 253, 170, 25, 4, 219], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 25, 245], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 25, 34], OperandSize::Dword)
}

#[test]
fn vbroadcastsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 253, 207, 25, 227], OperandSize::Qword)
}

#[test]
fn vbroadcastsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectDisplaced(RCX, 525576427, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 206, 25, 145, 235, 168, 83, 31], OperandSize::Qword)
}

