use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 153, 67, 199], OperandSize::Dword)
}

#[test]
fn vgetexpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 143, 67, 44, 64], OperandSize::Dword)
}

#[test]
fn vgetexpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 53, 153, 67, 207], OperandSize::Qword)
}

#[test]
fn vgetexpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 117, 141, 67, 60, 115], OperandSize::Qword)
}

