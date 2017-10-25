use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 242, 203], OperandSize::Dword)
}

#[test]
fn andn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: Some(IndirectDisplaced(EBX, 612314140, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 242, 187, 28, 44, 127, 36], OperandSize::Dword)
}

#[test]
fn andn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 201], OperandSize::Qword)
}

#[test]
fn andn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: Some(IndirectDisplaced(RBX, 1879154219, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 242, 155, 43, 158, 1, 112], OperandSize::Qword)
}

#[test]
fn andn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 242, 207], OperandSize::Qword)
}

#[test]
fn andn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1371566376, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 208, 242, 140, 79, 40, 113, 192, 81], OperandSize::Qword)
}

