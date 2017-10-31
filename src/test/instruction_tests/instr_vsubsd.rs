use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 92, 201], OperandSize::Dword)
}

#[test]
fn vsubsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 523422296, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 92, 12, 213, 88, 202, 50, 31], OperandSize::Dword)
}

#[test]
fn vsubsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 92, 195], OperandSize::Qword)
}

#[test]
fn vsubsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 653904725, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 92, 140, 207, 85, 203, 249, 38], OperandSize::Qword)
}

#[test]
fn vsubsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 199, 156, 92, 210], OperandSize::Dword)
}

#[test]
fn vsubsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1868870863, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 247, 141, 92, 4, 197, 207, 180, 100, 111], OperandSize::Dword)
}

#[test]
fn vsubsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 183, 151, 92, 219], OperandSize::Qword)
}

#[test]
fn vsubsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 215, 132, 92, 20, 120], OperandSize::Qword)
}

