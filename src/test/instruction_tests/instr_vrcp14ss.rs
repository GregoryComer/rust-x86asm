use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 77, 255], OperandSize::Dword)
}

#[test]
fn vrcp14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 615496242, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 77, 135, 50, 186, 175, 36], OperandSize::Dword)
}

#[test]
fn vrcp14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 53, 133, 77, 231], OperandSize::Qword)
}

#[test]
fn vrcp14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1910992327, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 101, 132, 77, 188, 134, 199, 109, 231, 113], OperandSize::Qword)
}

