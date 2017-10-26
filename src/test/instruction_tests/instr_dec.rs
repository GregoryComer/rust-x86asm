use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[75], OperandSize::Word)
}

#[test]
fn dec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 79], OperandSize::Dword)
}

#[test]
fn dec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 73], OperandSize::Word)
}

#[test]
fn dec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[77], OperandSize::Dword)
}

#[test]
fn dec_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 201], OperandSize::Word)
}

#[test]
fn dec_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Memory(25309, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 14, 221, 98], OperandSize::Word)
}

#[test]
fn dec_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 202], OperandSize::Dword)
}

#[test]
fn dec_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1008314565, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 12, 245, 197, 168, 25, 60], OperandSize::Dword)
}

#[test]
fn dec_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 203], OperandSize::Qword)
}

#[test]
fn dec_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(RDI, 1844980564, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 143, 84, 43, 248, 109], OperandSize::Qword)
}

#[test]
fn dec_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 202], OperandSize::Qword)
}

#[test]
fn dec_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1345181063, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 140, 113, 135, 213, 45, 80], OperandSize::Qword)
}

#[test]
fn dec_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[78], OperandSize::Word)
}

#[test]
fn dec_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 8], OperandSize::Word)
}

#[test]
fn dec_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 76], OperandSize::Dword)
}

#[test]
fn dec_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(ECX, 73747771, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 137, 59, 77, 101, 4], OperandSize::Dword)
}

#[test]
fn dec_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 201], OperandSize::Qword)
}

#[test]
fn dec_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 11], OperandSize::Qword)
}

#[test]
fn dec_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 79], OperandSize::Word)
}

#[test]
fn dec_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(BP, 44, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 78, 44], OperandSize::Word)
}

#[test]
fn dec_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[77], OperandSize::Dword)
}

#[test]
fn dec_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 12, 118], OperandSize::Dword)
}

#[test]
fn dec_23() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 205], OperandSize::Qword)
}

#[test]
fn dec_24() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 558773397, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 12, 221, 149, 52, 78, 33], OperandSize::Qword)
}

#[test]
fn dec_25() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 204], OperandSize::Qword)
}

#[test]
fn dec_26() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 12, 122], OperandSize::Qword)
}

