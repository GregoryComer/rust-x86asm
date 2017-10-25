use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 221], OperandSize::Word)
}

fn cmovg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 34], OperandSize::Word)
}

fn cmovg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 201], OperandSize::Dword)
}

fn cmovg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EBX, 1348324120, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 187, 24, 203, 93, 80], OperandSize::Dword)
}

fn cmovg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 211], OperandSize::Qword)
}

fn cmovg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(BP)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 46], OperandSize::Qword)
}

fn cmovg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 235], OperandSize::Word)
}

fn cmovg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 26001, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 153, 145, 101], OperandSize::Word)
}

fn cmovg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 221], OperandSize::Dword)
}

fn cmovg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 2051774481, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 180, 137, 17, 152, 75, 122], OperandSize::Dword)
}

fn cmovg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 234], OperandSize::Qword)
}

fn cmovg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 17], OperandSize::Qword)
}

fn cmovg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RSP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 225], OperandSize::Qword)
}

fn cmovg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVG, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1721336693, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 188, 150, 117, 131, 153, 102], OperandSize::Qword)
}

