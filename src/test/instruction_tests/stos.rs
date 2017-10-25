use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn stos_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 250, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Word)
}

fn stos_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Dword)
}

fn stos_3() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1030090016, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[170], OperandSize::Qword)
}

fn stos_4() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Word)
}

fn stos_5() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Dword)
}

fn stos_6() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Qword)
}

fn stos_7() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(DI, 188, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 171], OperandSize::Word)
}

fn stos_8() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Dword)
}

fn stos_9() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[171], OperandSize::Qword)
}

fn stos_10() {
    run_test(&Instruction { mnemonic: Mnemonic::STOS, operand1: Some(IndirectDisplaced(RDX, 1367456259, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 171], OperandSize::Qword)
}

