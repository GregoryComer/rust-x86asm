use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 242, 254], OperandSize::Dword)
}

#[test]
fn andn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 2074527248, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 242, 188, 131, 16, 198, 166, 123], OperandSize::Dword)
}

#[test]
fn andn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 242, 246], OperandSize::Qword)
}

#[test]
fn andn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: Some(IndirectDisplaced(RDI, 1519401875, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 242, 167, 147, 59, 144, 90], OperandSize::Qword)
}

#[test]
fn andn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 242, 255], OperandSize::Qword)
}

#[test]
fn andn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSP)), operand3: Some(IndirectDisplaced(RAX, 1991071476, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 242, 168, 244, 86, 173, 118], OperandSize::Qword)
}

