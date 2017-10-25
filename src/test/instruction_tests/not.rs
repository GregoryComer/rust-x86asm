use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn not_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 209], OperandSize::Word)
}

fn not_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 241, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 146, 241, 0], OperandSize::Word)
}

fn not_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 210], OperandSize::Dword)
}

fn not_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(EDI, 1399738204, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 151, 92, 79, 110, 83], OperandSize::Dword)
}

fn not_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 209], OperandSize::Qword)
}

fn not_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(RDX, 1404896243, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 146, 243, 3, 189, 83], OperandSize::Qword)
}

fn not_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 209], OperandSize::Qword)
}

fn not_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(RDX, 1861463443, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 146, 147, 173, 243, 110], OperandSize::Qword)
}

fn not_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 211], OperandSize::Word)
}

fn not_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 21], OperandSize::Word)
}

fn not_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 210], OperandSize::Dword)
}

fn not_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 22], OperandSize::Dword)
}

fn not_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 212], OperandSize::Qword)
}

fn not_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 17], OperandSize::Qword)
}

fn not_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 209], OperandSize::Word)
}

fn not_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 19], OperandSize::Word)
}

fn not_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 212], OperandSize::Dword)
}

fn not_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectScaledDisplaced(EDX, Four, 284581803, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 20, 149, 171, 95, 246, 16], OperandSize::Dword)
}

fn not_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 210], OperandSize::Qword)
}

fn not_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(RSI, 253436144, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 150, 240, 32, 27, 15], OperandSize::Qword)
}

fn not_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(Direct(RBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 213], OperandSize::Qword)
}

fn not_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NOT, operand1: Some(IndirectDisplaced(RBX, 1039516969, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 147, 41, 197, 245, 61], OperandSize::Qword)
}

