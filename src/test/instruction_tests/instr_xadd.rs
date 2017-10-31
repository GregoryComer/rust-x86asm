use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 218], OperandSize::Word)
}

#[test]
fn xadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 25], OperandSize::Word)
}

#[test]
fn xadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 209], OperandSize::Dword)
}

#[test]
fn xadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(ECX, 1518237586, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 145, 146, 119, 126, 90], OperandSize::Dword)
}

#[test]
fn xadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 202], OperandSize::Qword)
}

#[test]
fn xadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 19], OperandSize::Qword)
}

#[test]
fn xadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 202], OperandSize::Qword)
}

#[test]
fn xadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1667318953, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 140, 242, 169, 68, 97, 99], OperandSize::Qword)
}

#[test]
fn xadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 218], OperandSize::Word)
}

#[test]
fn xadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(BX, 25885, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 151, 29, 101], OperandSize::Word)
}

#[test]
fn xadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 203], OperandSize::Dword)
}

#[test]
fn xadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1443126220, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 164, 83, 204, 91, 4, 86], OperandSize::Dword)
}

#[test]
fn xadd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 222], OperandSize::Qword)
}

#[test]
fn xadd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 735352891, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 188, 94, 59, 152, 212, 43], OperandSize::Qword)
}

#[test]
fn xadd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 213], OperandSize::Word)
}

#[test]
fn xadd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 14, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 73, 14], OperandSize::Word)
}

#[test]
fn xadd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 227], OperandSize::Dword)
}

#[test]
fn xadd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1271639723, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 148, 150, 171, 174, 203, 75], OperandSize::Dword)
}

#[test]
fn xadd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 220], OperandSize::Qword)
}

#[test]
fn xadd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(RCX, 1340036517, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 161, 165, 85, 223, 79], OperandSize::Qword)
}

#[test]
fn xadd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 226], OperandSize::Qword)
}

#[test]
fn xadd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 796525379, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 12, 197, 67, 3, 122, 47], OperandSize::Qword)
}

