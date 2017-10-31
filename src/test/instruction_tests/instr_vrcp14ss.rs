use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 77, 196], OperandSize::Dword)
}

#[test]
fn vrcp14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 77, 27], OperandSize::Dword)
}

#[test]
fn vrcp14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 29, 129, 77, 229], OperandSize::Qword)
}

#[test]
fn vrcp14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 101, 129, 77, 6], OperandSize::Qword)
}

