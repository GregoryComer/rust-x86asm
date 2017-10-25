use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 212], OperandSize::Word)
}

fn cmovnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 176, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 184, 176, 0], OperandSize::Word)
}

fn cmovnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 245], OperandSize::Dword)
}

fn cmovnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(BX)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 26], OperandSize::Dword)
}

fn cmovnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 210], OperandSize::Qword)
}

fn cmovnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 12, 72], OperandSize::Qword)
}

fn cmovnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 244], OperandSize::Word)
}

fn cmovnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 8], OperandSize::Word)
}

fn cmovnl_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 213], OperandSize::Dword)
}

fn cmovnl_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1313921813, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 164, 193, 21, 219, 80, 78], OperandSize::Dword)
}

fn cmovnl_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 202], OperandSize::Qword)
}

fn cmovnl_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 2061609544, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 20, 69, 72, 170, 225, 122], OperandSize::Qword)
}

fn cmovnl_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 245], OperandSize::Qword)
}

fn cmovnl_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNL, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 52, 254], OperandSize::Qword)
}

