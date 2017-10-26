use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 234], OperandSize::Word)
}

#[test]
fn cvtpi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 188, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 185, 188, 0], OperandSize::Word)
}

#[test]
fn cvtpi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 211], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1983901841, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 28, 205, 145, 240, 63, 118], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 202], OperandSize::Qword)
}

#[test]
fn cvtpi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 4, 79], OperandSize::Qword)
}

