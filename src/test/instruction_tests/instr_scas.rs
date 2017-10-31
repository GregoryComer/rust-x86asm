use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn scas_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 81, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[174], OperandSize::Word)
}

#[test]
fn scas_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[174], OperandSize::Dword)
}

#[test]
fn scas_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledDisplaced(RDI, Four, 26997532, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[174], OperandSize::Qword)
}

#[test]
fn scas_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[175], OperandSize::Word)
}

#[test]
fn scas_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 175], OperandSize::Dword)
}

#[test]
fn scas_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1129934301, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 175], OperandSize::Qword)
}

#[test]
fn scas_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 32, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 175], OperandSize::Word)
}

#[test]
fn scas_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectDisplaced(EAX, 1079366424, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[175], OperandSize::Dword)
}

#[test]
fn scas_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[175], OperandSize::Qword)
}

#[test]
fn scas_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1007697411, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 175], OperandSize::Qword)
}

