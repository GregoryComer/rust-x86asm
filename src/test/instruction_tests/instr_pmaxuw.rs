use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 216], OperandSize::Dword)
}

#[test]
fn pmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1313663103, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 60, 77, 127, 232, 76, 78], OperandSize::Dword)
}

#[test]
fn pmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 224], OperandSize::Qword)
}

#[test]
fn pmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1655837975, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 60, 189, 23, 21, 178, 98], OperandSize::Qword)
}

