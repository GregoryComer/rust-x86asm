use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 157, 39, 250, 108], OperandSize::Dword)
}

#[test]
fn vgetmantss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 1448475370, Some(OperandSize::Dword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 93, 143, 39, 182, 234, 250, 85, 86, 14], OperandSize::Dword)
}

#[test]
fn vgetmantss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 83, 77, 145, 39, 193, 1], OperandSize::Qword)
}

#[test]
fn vgetmantss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 901269040, Some(OperandSize::Dword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 109, 137, 39, 148, 194, 48, 70, 184, 53, 53], OperandSize::Qword)
}

