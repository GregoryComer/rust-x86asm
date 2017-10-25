use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn scas_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 135, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[174], OperandSize::Word)
}

fn scas_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[174], OperandSize::Dword)
}

fn scas_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 789454013, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[174], OperandSize::Qword)
}

fn scas_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectDisplaced(BP, 22108, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[175], OperandSize::Word)
}

fn scas_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 175], OperandSize::Dword)
}

fn scas_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 175], OperandSize::Qword)
}

fn scas_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 175], OperandSize::Word)
}

fn scas_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[175], OperandSize::Dword)
}

fn scas_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[175], OperandSize::Qword)
}

fn scas_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SCAS, operand1: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 175], OperandSize::Qword)
}

