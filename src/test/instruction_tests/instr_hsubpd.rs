use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn hsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 223], OperandSize::Dword)
}

#[test]
fn hsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 28, 217], OperandSize::Dword)
}

#[test]
fn hsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 224], OperandSize::Qword)
}

#[test]
fn hsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RBX, 1439738018, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 125, 163, 162, 168, 208, 85], OperandSize::Qword)
}

