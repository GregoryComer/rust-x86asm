use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 138, 77, 252], OperandSize::Dword)
}

#[test]
fn vrcp14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 77, 20, 208], OperandSize::Dword)
}

#[test]
fn vrcp14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 21, 132, 77, 208], OperandSize::Qword)
}

#[test]
fn vrcp14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 85, 142, 77, 49], OperandSize::Qword)
}

