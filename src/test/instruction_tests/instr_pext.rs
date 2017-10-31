use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pext_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 245, 233], OperandSize::Dword)
}

#[test]
fn pext_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 374637822, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 245, 164, 120, 254, 132, 84, 22], OperandSize::Dword)
}

#[test]
fn pext_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 245, 203], OperandSize::Qword)
}

#[test]
fn pext_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1150552503, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 245, 180, 198, 183, 9, 148, 68], OperandSize::Qword)
}

#[test]
fn pext_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 234, 245, 213], OperandSize::Qword)
}

#[test]
fn pext_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PEXT, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 245, 20, 83], OperandSize::Qword)
}

