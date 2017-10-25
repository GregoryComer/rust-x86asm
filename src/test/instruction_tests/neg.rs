use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn neg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Word)
}

fn neg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(BP, 219, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 158, 219, 0], OperandSize::Word)
}

fn neg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 219], OperandSize::Dword)
}

fn neg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(EBX, 970803269, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 155, 69, 72, 221, 57], OperandSize::Dword)
}

fn neg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 217], OperandSize::Qword)
}

fn neg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1936396919, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 28, 213, 119, 18, 107, 115], OperandSize::Qword)
}

fn neg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 219], OperandSize::Qword)
}

fn neg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 729260738, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 156, 190, 194, 162, 119, 43], OperandSize::Qword)
}

fn neg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 222], OperandSize::Word)
}

fn neg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(BP, 168, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 158, 168, 0], OperandSize::Word)
}

fn neg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 221], OperandSize::Dword)
}

fn neg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 28, 246], OperandSize::Dword)
}

fn neg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 223], OperandSize::Qword)
}

fn neg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1542514236, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 156, 127, 60, 230, 240, 91], OperandSize::Qword)
}

fn neg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 223], OperandSize::Word)
}

fn neg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 25], OperandSize::Word)
}

fn neg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 217], OperandSize::Dword)
}

fn neg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(EDX, Four, 2087672817, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 28, 149, 241, 91, 111, 124], OperandSize::Dword)
}

fn neg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 217], OperandSize::Qword)
}

fn neg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(RDI, 1367383088, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 159, 48, 156, 128, 81], OperandSize::Qword)
}

fn neg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 220], OperandSize::Qword)
}

fn neg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(RAX, Two, 982833472, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 28, 69, 64, 217, 148, 58], OperandSize::Qword)
}

