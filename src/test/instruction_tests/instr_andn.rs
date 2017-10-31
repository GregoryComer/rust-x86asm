use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 225], OperandSize::Dword)
}

#[test]
fn andn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 242, 36, 159], OperandSize::Dword)
}

#[test]
fn andn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 233], OperandSize::Qword)
}

#[test]
fn andn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 40544682, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 44, 157, 170, 169, 106, 2], OperandSize::Qword)
}

#[test]
fn andn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 224, 242, 231], OperandSize::Qword)
}

#[test]
fn andn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 224, 242, 59], OperandSize::Qword)
}

