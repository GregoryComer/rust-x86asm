use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movnti_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 154, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 195, 184, 154, 0], OperandSize::Word)
}

#[test]
fn movnti_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1654153492, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 12, 69, 20, 97, 152, 98], OperandSize::Dword)
}

#[test]
fn movnti_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 30], OperandSize::Qword)
}

#[test]
fn movnti_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 195, 52, 120], OperandSize::Qword)
}

