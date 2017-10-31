use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastf32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 25, 238], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 340025082, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 25, 60, 205, 250, 94, 68, 20], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM29)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 125, 171, 25, 233], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 541217455, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 174, 25, 172, 67, 175, 82, 66, 32], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 25, 245], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 25, 16], OperandSize::Dword)
}

#[test]
fn vbroadcastf32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 125, 202, 25, 232], OperandSize::Qword)
}

#[test]
fn vbroadcastf32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1905673821, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 25, 132, 134, 93, 70, 150, 113], OperandSize::Qword)
}

