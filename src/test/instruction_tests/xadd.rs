use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 210], OperandSize::Word)
}

fn xadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 17], OperandSize::Word)
}

fn xadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 209], OperandSize::Dword)
}

fn xadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(ECX, 1434598023, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 145, 135, 58, 130, 85], OperandSize::Dword)
}

fn xadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 202], OperandSize::Qword)
}

fn xadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(RSI, 1341665325, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 158, 45, 48, 248, 79], OperandSize::Qword)
}

fn xadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 209], OperandSize::Qword)
}

fn xadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 806597074, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 140, 75, 210, 177, 19, 48], OperandSize::Qword)
}

fn xadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 247], OperandSize::Word)
}

fn xadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(SI, 10350, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 172, 110, 40], OperandSize::Word)
}

fn xadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 229], OperandSize::Dword)
}

fn xadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 14], OperandSize::Dword)
}

fn xadd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 226], OperandSize::Qword)
}

fn xadd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 36, 187], OperandSize::Qword)
}

fn xadd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 213], OperandSize::Word)
}

fn xadd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Memory(31657, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 54, 169, 123], OperandSize::Word)
}

fn xadd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 203], OperandSize::Dword)
}

fn xadd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 28, 138], OperandSize::Dword)
}

fn xadd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 255], OperandSize::Qword)
}

fn xadd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 24], OperandSize::Qword)
}

fn xadd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 255], OperandSize::Qword)
}

fn xadd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 33], OperandSize::Qword)
}

