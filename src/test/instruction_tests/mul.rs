use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn mul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 227], OperandSize::Word)
}

fn mul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 55, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 99, 55], OperandSize::Word)
}

fn mul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 226], OperandSize::Dword)
}

fn mul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(EDI, 1444369855, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 167, 191, 85, 23, 86], OperandSize::Dword)
}

fn mul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 226], OperandSize::Qword)
}

fn mul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 36, 121], OperandSize::Qword)
}

fn mul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 226], OperandSize::Qword)
}

fn mul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(RSI, 1072159045, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 166, 69, 217, 231, 63], OperandSize::Qword)
}

fn mul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 227], OperandSize::Word)
}

fn mul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(DI, 29696, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 165, 0, 116], OperandSize::Word)
}

fn mul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 231], OperandSize::Dword)
}

fn mul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 36, 208], OperandSize::Dword)
}

fn mul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 228], OperandSize::Qword)
}

fn mul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 38], OperandSize::Qword)
}

fn mul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 230], OperandSize::Word)
}

fn mul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 32], OperandSize::Word)
}

fn mul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 230], OperandSize::Dword)
}

fn mul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectDisplaced(EAX, 1695583051, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 160, 75, 139, 16, 101], OperandSize::Dword)
}

fn mul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 225], OperandSize::Qword)
}

fn mul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1627629685, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 36, 141, 117, 168, 3, 97], OperandSize::Qword)
}

fn mul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 230], OperandSize::Qword)
}

fn mul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MUL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1935046246, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 164, 118, 102, 118, 86, 115], OperandSize::Qword)
}

