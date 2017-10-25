use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn popcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 234], OperandSize::Word)
}

fn popcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 92, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 123, 92], OperandSize::Word)
}

fn popcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 244], OperandSize::Dword)
}

fn popcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 826079498, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 164, 152, 10, 249, 60, 49], OperandSize::Dword)
}

fn popcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 236], OperandSize::Qword)
}

fn popcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(BP)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 46], OperandSize::Qword)
}

fn popcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 228], OperandSize::Word)
}

fn popcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EDX)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 184, 23], OperandSize::Word)
}

fn popcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 229], OperandSize::Dword)
}

fn popcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 197047415, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 12, 205, 119, 180, 190, 11], OperandSize::Dword)
}

fn popcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 244], OperandSize::Qword)
}

fn popcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RAX, 492327615, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 184, 152, 191, 82, 88, 29], OperandSize::Qword)
}

fn popcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 205], OperandSize::Qword)
}

fn popcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::POPCNT, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 911026596, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 184, 20, 85, 164, 41, 77, 54], OperandSize::Qword)
}

