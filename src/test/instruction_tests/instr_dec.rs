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
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 76], OperandSize::Dword)
}

#[test]
fn dec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 76], OperandSize::Word)
}

#[test]
fn dec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[73], OperandSize::Dword)
}

#[test]
fn dec_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 203], OperandSize::Word)
}

#[test]
fn dec_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(DI, 29018, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 141, 90, 113], OperandSize::Word)
}

#[test]
fn dec_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 203], OperandSize::Dword)
}

#[test]
fn dec_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 15], OperandSize::Dword)
}

#[test]
fn dec_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 201], OperandSize::Qword)
}

#[test]
fn dec_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1858456190, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 140, 128, 126, 202, 197, 110], OperandSize::Qword)
}

#[test]
fn dec_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 202], OperandSize::Qword)
}

#[test]
fn dec_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 12, 153], OperandSize::Qword)
}

#[test]
fn dec_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[76], OperandSize::Word)
}

#[test]
fn dec_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 5167, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 136, 47, 20], OperandSize::Word)
}

#[test]
fn dec_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 77], OperandSize::Dword)
}

#[test]
fn dec_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 12, 112], OperandSize::Dword)
}

#[test]
fn dec_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 202], OperandSize::Qword)
}

#[test]
fn dec_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 10], OperandSize::Qword)
}

#[test]
fn dec_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 74], OperandSize::Word)
}

#[test]
fn dec_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 58, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 74, 58], OperandSize::Word)
}

#[test]
fn dec_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[79], OperandSize::Dword)
}

#[test]
fn dec_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(ESI, 311025521, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 142, 113, 223, 137, 18], OperandSize::Dword)
}

#[test]
fn dec_23() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 204], OperandSize::Qword)
}

#[test]
fn dec_24() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1734036482, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 12, 77, 2, 76, 91, 103], OperandSize::Qword)
}

#[test]
fn dec_25() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(RDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 202], OperandSize::Qword)
}

#[test]
fn dec_26() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 12, 151], OperandSize::Qword)
}

