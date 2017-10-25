use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovns_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 211], OperandSize::Word)
}

fn cmovns_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(BX)), operand2: Some(Memory(9492, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 30, 20, 37], OperandSize::Word)
}

fn cmovns_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 250], OperandSize::Dword)
}

fn cmovns_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 52, 123], OperandSize::Dword)
}

fn cmovns_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 203], OperandSize::Qword)
}

fn cmovns_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 809183283, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 148, 203, 51, 40, 59, 48], OperandSize::Qword)
}

fn cmovns_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 211], OperandSize::Word)
}

fn cmovns_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 228, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 73, 178, 228, 0], OperandSize::Word)
}

fn cmovns_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 204], OperandSize::Dword)
}

fn cmovns_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 2015737177, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 28, 205, 89, 181, 37, 120], OperandSize::Dword)
}

fn cmovns_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 252], OperandSize::Qword)
}

fn cmovns_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 808131384, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 73, 36, 245, 56, 27, 43, 48], OperandSize::Qword)
}

fn cmovns_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RCX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 201], OperandSize::Qword)
}

fn cmovns_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNS, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 73, 20, 153], OperandSize::Qword)
}

