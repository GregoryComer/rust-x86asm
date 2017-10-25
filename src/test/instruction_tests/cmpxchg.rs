use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmpxchg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 218], OperandSize::Word)
}

fn cmpxchg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(BX, 2132, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 159, 84, 8], OperandSize::Word)
}

fn cmpxchg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 218], OperandSize::Dword)
}

fn cmpxchg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 302595767, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 156, 118, 183, 62, 9, 18], OperandSize::Dword)
}

fn cmpxchg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 202], OperandSize::Qword)
}

fn cmpxchg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(RDI, Two, 2091377662, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 20, 125, 254, 227, 167, 124], OperandSize::Qword)
}

fn cmpxchg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 203], OperandSize::Qword)
}

fn cmpxchg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 725028548, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 176, 28, 245, 196, 14, 55, 43], OperandSize::Qword)
}

fn cmpxchg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 213], OperandSize::Word)
}

fn cmpxchg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectDisplaced(SI, 4088, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 188, 248, 15], OperandSize::Word)
}

fn cmpxchg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 226], OperandSize::Dword)
}

fn cmpxchg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledDisplaced(ESI, Two, 654255617, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 60, 117, 1, 38, 255, 38], OperandSize::Dword)
}

fn cmpxchg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 210], OperandSize::Qword)
}

fn cmpxchg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 34], OperandSize::Qword)
}

fn cmpxchg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 255], OperandSize::Word)
}

fn cmpxchg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 177, 34], OperandSize::Word)
}

fn cmpxchg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 252], OperandSize::Dword)
}

fn cmpxchg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 14], OperandSize::Dword)
}

fn cmpxchg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 252], OperandSize::Qword)
}

fn cmpxchg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 177, 36, 142], OperandSize::Qword)
}

fn cmpxchg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 235], OperandSize::Qword)
}

fn cmpxchg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPXCHG, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 177, 60, 207], OperandSize::Qword)
}

