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
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 217], OperandSize::Dword)
}

#[test]
fn neg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 28, 118], OperandSize::Dword)
}

#[test]
fn neg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Qword)
}

#[test]
fn neg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1688044826, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 28, 157, 26, 133, 157, 100], OperandSize::Qword)
}

#[test]
fn neg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 217], OperandSize::Qword)
}

#[test]
fn neg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(RDI, 304870030, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 159, 142, 242, 43, 18], OperandSize::Qword)
}

#[test]
fn neg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 221], OperandSize::Word)
}

#[test]
fn neg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 31], OperandSize::Word)
}

#[test]
fn neg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 222], OperandSize::Dword)
}

#[test]
fn neg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(EBX, 941192607, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 155, 159, 117, 25, 56], OperandSize::Dword)
}

#[test]
fn neg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 219], OperandSize::Qword)
}

#[test]
fn neg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1398312168, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 156, 129, 232, 140, 88, 83], OperandSize::Qword)
}

#[test]
fn neg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 218], OperandSize::Word)
}

#[test]
fn neg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 26], OperandSize::Word)
}

#[test]
fn neg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 221], OperandSize::Dword)
}

#[test]
fn neg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 28, 199], OperandSize::Dword)
}

#[test]
fn neg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 219], OperandSize::Qword)
}

#[test]
fn neg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 28, 145], OperandSize::Qword)
}

#[test]
fn neg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 222], OperandSize::Qword)
}

#[test]
fn neg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 28, 74], OperandSize::Qword)
}

