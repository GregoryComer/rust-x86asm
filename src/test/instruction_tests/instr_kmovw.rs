use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K7)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 251], OperandSize::Dword)
}

#[test]
fn kmovw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K2)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 17], OperandSize::Dword)
}

#[test]
fn kmovw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K2)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 215], OperandSize::Qword)
}

#[test]
fn kmovw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 44, 186], OperandSize::Qword)
}

#[test]
fn kmovw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectDisplaced(EBX, 34101637, Some(OperandSize::Word), None)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 155, 133, 89, 8, 2], OperandSize::Dword)
}

#[test]
fn kmovw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 314663952, Some(OperandSize::Word), None)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 44, 197, 16, 100, 193, 18], OperandSize::Qword)
}

#[test]
fn kmovw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K4)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 227], OperandSize::Dword)
}

#[test]
fn kmovw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K4)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 227], OperandSize::Qword)
}

#[test]
fn kmovw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(ESI)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 243], OperandSize::Dword)
}

#[test]
fn kmovw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(EBP)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 235], OperandSize::Qword)
}

