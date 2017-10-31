use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 77, 236], OperandSize::Dword)
}

#[test]
fn vrcp14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1876480232, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 77, 180, 183, 232, 208, 216, 111], OperandSize::Dword)
}

#[test]
fn vrcp14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 109, 133, 77, 200], OperandSize::Qword)
}

#[test]
fn vrcp14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 69, 140, 77, 38], OperandSize::Qword)
}

