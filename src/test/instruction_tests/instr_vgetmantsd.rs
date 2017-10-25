use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 221, 153, 39, 228, 79], OperandSize::Dword)
}

#[test]
fn vgetmantsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 205, 143, 39, 41, 2], OperandSize::Dword)
}

#[test]
fn vgetmantsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM25)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 131, 221, 145, 39, 201, 76], OperandSize::Qword)
}

#[test]
fn vgetmantsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RDI, 1538795943, Some(OperandSize::Qword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 149, 137, 39, 143, 167, 41, 184, 91, 76], OperandSize::Qword)
}

