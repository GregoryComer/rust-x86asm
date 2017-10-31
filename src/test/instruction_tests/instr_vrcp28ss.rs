use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 154, 203, 229], OperandSize::Dword)
}

#[test]
fn vrcp28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 138, 203, 52, 150], OperandSize::Dword)
}

#[test]
fn vrcp28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 29, 158, 203, 200], OperandSize::Qword)
}

#[test]
fn vrcp28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RAX, 1904667259, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 203, 176, 123, 234, 134, 113], OperandSize::Qword)
}

