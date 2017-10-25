use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 153, 67, 203], OperandSize::Dword)
}

#[test]
fn vgetexpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 67, 60, 195], OperandSize::Dword)
}

#[test]
fn vgetexpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 213, 151, 67, 206], OperandSize::Qword)
}

#[test]
fn vgetexpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 157, 131, 67, 31], OperandSize::Qword)
}

