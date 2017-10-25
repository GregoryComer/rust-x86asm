use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn neg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 217], OperandSize::Word)
}

#[test]
fn neg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 26], OperandSize::Word)
}

#[test]
fn neg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Dword)
}

#[test]
fn neg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 28, 241], OperandSize::Dword)
}

#[test]
fn neg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Qword)
}

#[test]
fn neg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(RBX, 315679053, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 155, 77, 225, 208, 18], OperandSize::Qword)
}

#[test]
fn neg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Qword)
}

#[test]
fn neg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(RDI, 198338118, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 159, 70, 102, 210, 11], OperandSize::Qword)
}

#[test]
fn neg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 221], OperandSize::Word)
}

#[test]
fn neg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(DI, 4734, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 157, 126, 18], OperandSize::Word)
}

#[test]
fn neg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 218], OperandSize::Dword)
}

#[test]
fn neg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1316073491, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 28, 125, 19, 176, 113, 78], OperandSize::Dword)
}

#[test]
fn neg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 222], OperandSize::Qword)
}

#[test]
fn neg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 31], OperandSize::Qword)
}

#[test]
fn neg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 222], OperandSize::Word)
}

#[test]
fn neg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 31], OperandSize::Word)
}

#[test]
fn neg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 217], OperandSize::Dword)
}

#[test]
fn neg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 31], OperandSize::Dword)
}

#[test]
fn neg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 221], OperandSize::Qword)
}

#[test]
fn neg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 762251204, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 156, 199, 196, 7, 111, 45], OperandSize::Qword)
}

#[test]
fn neg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(RCX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 217], OperandSize::Qword)
}

#[test]
fn neg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 26], OperandSize::Qword)
}

