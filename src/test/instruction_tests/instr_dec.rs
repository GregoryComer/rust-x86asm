use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[76], OperandSize::Word)
}

#[test]
fn dec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 74], OperandSize::Dword)
}

#[test]
fn dec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 76], OperandSize::Word)
}

#[test]
fn dec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[78], OperandSize::Dword)
}

#[test]
fn dec_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 201], OperandSize::Word)
}

#[test]
fn dec_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 13], OperandSize::Word)
}

#[test]
fn dec_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 203], OperandSize::Dword)
}

#[test]
fn dec_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 509056710, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 140, 185, 198, 150, 87, 30], OperandSize::Dword)
}

#[test]
fn dec_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 203], OperandSize::Qword)
}

#[test]
fn dec_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1612076534, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 140, 177, 246, 85, 22, 96], OperandSize::Qword)
}

#[test]
fn dec_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 201], OperandSize::Qword)
}

#[test]
fn dec_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 12, 75], OperandSize::Qword)
}

#[test]
fn dec_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[78], OperandSize::Word)
}

#[test]
fn dec_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 34, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 75, 34], OperandSize::Word)
}

#[test]
fn dec_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 74], OperandSize::Dword)
}

#[test]
fn dec_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1727221114, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 12, 253, 122, 77, 243, 102], OperandSize::Dword)
}

#[test]
fn dec_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 204], OperandSize::Qword)
}

#[test]
fn dec_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 849437491, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 12, 197, 51, 99, 161, 50], OperandSize::Qword)
}

#[test]
fn dec_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 75], OperandSize::Word)
}

#[test]
fn dec_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 9], OperandSize::Word)
}

#[test]
fn dec_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[73], OperandSize::Dword)
}

#[test]
fn dec_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(ESI, 78101556, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 142, 52, 188, 167, 4], OperandSize::Dword)
}

#[test]
fn dec_23() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 201], OperandSize::Qword)
}

#[test]
fn dec_24() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 12, 206], OperandSize::Qword)
}

#[test]
fn dec_25() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(RBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 203], OperandSize::Qword)
}

#[test]
fn dec_26() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(RDI, 110749676, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 143, 236, 231, 153, 6], OperandSize::Qword)
}

