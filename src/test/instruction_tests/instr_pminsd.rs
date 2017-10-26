use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 230], OperandSize::Dword)
}

#[test]
fn pminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 60, 150], OperandSize::Dword)
}

#[test]
fn pminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 245], OperandSize::Qword)
}

#[test]
fn pminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDI, 963174538, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 191, 138, 224, 104, 57], OperandSize::Qword)
}

