use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 137, 77, 249], OperandSize::Dword)
}

#[test]
fn vrcp14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 143, 77, 4, 70], OperandSize::Dword)
}

#[test]
fn vrcp14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 173, 143, 77, 242], OperandSize::Qword)
}

#[test]
fn vrcp14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1124472538, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 77, 4, 213, 218, 22, 6, 67], OperandSize::Qword)
}

