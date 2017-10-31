use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pdep_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 245, 223], OperandSize::Dword)
}

#[test]
fn pdep_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1657135626, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 245, 180, 138, 10, 226, 197, 98], OperandSize::Dword)
}

#[test]
fn pdep_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 107, 245, 241], OperandSize::Qword)
}

#[test]
fn pdep_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(IndirectDisplaced(RSI, 1281347526, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 245, 166, 198, 207, 95, 76], OperandSize::Qword)
}

#[test]
fn pdep_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 195, 245, 238], OperandSize::Qword)
}

#[test]
fn pdep_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: Some(IndirectDisplaced(RDI, 1251759622, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 235, 245, 175, 6, 86, 156, 74], OperandSize::Qword)
}

