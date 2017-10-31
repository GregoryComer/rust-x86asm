use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pdep_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 245, 218], OperandSize::Dword)
}

#[test]
fn pdep_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 444800528, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 99, 245, 164, 119, 16, 30, 131, 26], OperandSize::Dword)
}

#[test]
fn pdep_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 245, 228], OperandSize::Qword)
}

#[test]
fn pdep_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 923912191, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 245, 36, 189, 255, 199, 17, 55], OperandSize::Qword)
}

#[test]
fn pdep_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBP)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 211, 245, 247], OperandSize::Qword)
}

#[test]
fn pdep_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSI)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1242148224, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 245, 44, 181, 128, 173, 9, 74], OperandSize::Qword)
}

