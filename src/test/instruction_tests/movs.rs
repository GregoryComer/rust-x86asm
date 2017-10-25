use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Memory(25990, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(BX, 118, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Word)
}

fn movs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 2097938419, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Dword)
}

fn movs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(RAX, 1524004659, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[164], OperandSize::Qword)
}

fn movs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 44, Some(OperandSize::Word), None)), operand2: Some(IndirectDisplaced(BX, 9704, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Word)
}

fn movs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1881261114, Some(OperandSize::Word), None)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Dword)
}

fn movs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1362938573, Some(OperandSize::Word), None)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Qword)
}

fn movs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 29135, Some(OperandSize::Dword), None)), operand2: Some(IndirectDisplaced(SI, 16806, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 165], OperandSize::Word)
}

fn movs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Dword)
}

fn movs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(IndirectDisplaced(RBX, 969428722, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 13601403, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[165], OperandSize::Qword)
}

fn movs_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVS, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(IndirectDisplaced(RCX, 434447615, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 165], OperandSize::Qword)
}

