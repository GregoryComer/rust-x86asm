use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 153, 81, 252, 9], OperandSize::Dword)
}

#[test]
fn vrangesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 487884265, Some(OperandSize::Qword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 237, 140, 81, 4, 69, 233, 133, 20, 29, 80], OperandSize::Dword)
}

#[test]
fn vrangesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 197, 158, 81, 207, 74], OperandSize::Qword)
}

#[test]
fn vrangesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 133, 138, 81, 28, 74, 69], OperandSize::Qword)
}

