use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 199], OperandSize::Dword)
}

#[test]
fn pmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EBX, 1272979007, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 147, 63, 30, 224, 75], OperandSize::Dword)
}

#[test]
fn pmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 203], OperandSize::Qword)
}

#[test]
fn pmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 50], OperandSize::Qword)
}

