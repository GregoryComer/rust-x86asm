use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 219], OperandSize::Word)
}

#[test]
fn xadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 236, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 155, 236, 0], OperandSize::Word)
}

#[test]
fn xadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 210], OperandSize::Dword)
}

#[test]
fn xadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1024634730, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 20, 77, 106, 175, 18, 61], OperandSize::Dword)
}

#[test]
fn xadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 211], OperandSize::Qword)
}

#[test]
fn xadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 28, 126], OperandSize::Qword)
}

#[test]
fn xadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 218], OperandSize::Qword)
}

#[test]
fn xadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1099815293, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 192, 148, 143, 125, 217, 141, 65], OperandSize::Qword)
}

#[test]
fn xadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 242], OperandSize::Word)
}

#[test]
fn xadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Memory(30155, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 54, 203, 117], OperandSize::Word)
}

#[test]
fn xadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 254], OperandSize::Dword)
}

#[test]
fn xadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 8], OperandSize::Dword)
}

#[test]
fn xadd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 254], OperandSize::Qword)
}

#[test]
fn xadd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(RCX, Two, 750065183, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 12, 77, 31, 22, 181, 44], OperandSize::Qword)
}

#[test]
fn xadd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 201], OperandSize::Word)
}

#[test]
fn xadd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 193, 36], OperandSize::Word)
}

#[test]
fn xadd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 247], OperandSize::Dword)
}

#[test]
fn xadd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectDisplaced(EBX, 2019150721, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 139, 129, 203, 89, 120], OperandSize::Dword)
}

#[test]
fn xadd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 230], OperandSize::Qword)
}

#[test]
fn xadd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 193, 36, 94], OperandSize::Qword)
}

#[test]
fn xadd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 244], OperandSize::Qword)
}

#[test]
fn xadd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XADD, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1344112946, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 193, 12, 93, 50, 137, 29, 80], OperandSize::Qword)
}

