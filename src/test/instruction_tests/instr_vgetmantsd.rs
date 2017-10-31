use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 157, 39, 226, 87], OperandSize::Dword)
}

#[test]
fn vgetmantsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 546548132, Some(OperandSize::Qword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 141, 39, 148, 199, 164, 169, 147, 32, 76], OperandSize::Dword)
}

#[test]
fn vgetmantsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 181, 148, 39, 212, 127], OperandSize::Qword)
}

#[test]
fn vgetmantsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 213, 129, 39, 32, 13], OperandSize::Qword)
}

