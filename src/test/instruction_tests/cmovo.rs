use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovo_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 212], OperandSize::Word)
}

fn cmovo_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(Memory(26950, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 46, 70, 105], OperandSize::Word)
}

fn cmovo_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 236], OperandSize::Dword)
}

fn cmovo_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 28, 249], OperandSize::Dword)
}

fn cmovo_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 246], OperandSize::Qword)
}

fn cmovo_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 46], OperandSize::Qword)
}

fn cmovo_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 209], OperandSize::Word)
}

fn cmovo_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(BX, 94, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 79, 94], OperandSize::Word)
}

fn cmovo_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 234], OperandSize::Dword)
}

fn cmovo_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 36, 65], OperandSize::Dword)
}

fn cmovo_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 218], OperandSize::Qword)
}

fn cmovo_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 20, 206], OperandSize::Qword)
}

fn cmovo_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 243], OperandSize::Qword)
}

fn cmovo_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1913693153, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 36, 197, 225, 163, 16, 114], OperandSize::Qword)
}

