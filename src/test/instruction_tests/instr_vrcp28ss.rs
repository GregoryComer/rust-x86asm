use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 159, 203, 249], OperandSize::Dword)
}

#[test]
fn vrcp28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1780382507, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 203, 52, 245, 43, 123, 30, 106], OperandSize::Dword)
}

#[test]
fn vrcp28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 125, 156, 203, 199], OperandSize::Qword)
}

#[test]
fn vrcp28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 85, 138, 203, 28, 130], OperandSize::Qword)
}

