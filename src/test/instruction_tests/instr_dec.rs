use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn dec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[78], OperandSize::Word)
}

#[test]
fn dec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 77], OperandSize::Dword)
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
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 202], OperandSize::Word)
}

#[test]
fn dec_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 12], OperandSize::Word)
}

#[test]
fn dec_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 202], OperandSize::Dword)
}

#[test]
fn dec_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 12, 240], OperandSize::Dword)
}

#[test]
fn dec_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 202], OperandSize::Qword)
}

#[test]
fn dec_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 14], OperandSize::Qword)
}

#[test]
fn dec_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 201], OperandSize::Qword)
}

#[test]
fn dec_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 488366528, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[254, 140, 195, 192, 225, 27, 29], OperandSize::Qword)
}

#[test]
fn dec_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[73], OperandSize::Word)
}

#[test]
fn dec_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 12], OperandSize::Word)
}

#[test]
fn dec_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 77], OperandSize::Dword)
}

#[test]
fn dec_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1526510094, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 140, 67, 14, 178, 252, 90], OperandSize::Dword)
}

#[test]
fn dec_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 207], OperandSize::Qword)
}

#[test]
fn dec_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 9], OperandSize::Qword)
}

#[test]
fn dec_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 76], OperandSize::Word)
}

#[test]
fn dec_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 255, 8], OperandSize::Word)
}

#[test]
fn dec_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[78], OperandSize::Dword)
}

#[test]
fn dec_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 12, 222], OperandSize::Dword)
}

#[test]
fn dec_23() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 204], OperandSize::Qword)
}

#[test]
fn dec_24() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectDisplaced(RSI, 975184138, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[255, 142, 10, 33, 32, 58], OperandSize::Qword)
}

#[test]
fn dec_25() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 204], OperandSize::Qword)
}

#[test]
fn dec_26() {
    run_test(&Instruction { mnemonic: Mnemonic::DEC, operand1: Some(IndirectScaledDisplaced(RCX, Two, 365508591, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 255, 12, 77, 239, 55, 201, 21], OperandSize::Qword)
}

