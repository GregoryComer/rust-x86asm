use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 210], OperandSize::Word)
}

fn cmovnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BP)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 44], OperandSize::Word)
}

fn cmovnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 205], OperandSize::Dword)
}

fn cmovnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 12, 134], OperandSize::Dword)
}

fn cmovnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 249], OperandSize::Qword)
}

fn cmovnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 56512191, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 156, 190, 191, 78, 94, 3], OperandSize::Qword)
}

fn cmovnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 229], OperandSize::Word)
}

fn cmovnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 43], OperandSize::Word)
}

fn cmovnc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 213], OperandSize::Dword)
}

fn cmovnc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 28, 241], OperandSize::Dword)
}

fn cmovnc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 228], OperandSize::Qword)
}

fn cmovnc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 605377397, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 52, 133, 117, 83, 21, 36], OperandSize::Qword)
}

fn cmovnc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 246], OperandSize::Qword)
}

fn cmovnc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNC, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 616836941, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 44, 125, 77, 47, 196, 36], OperandSize::Qword)
}

