use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn neg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Word)
}

#[test]
fn neg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 31], OperandSize::Word)
}

#[test]
fn neg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Dword)
}

#[test]
fn neg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 28, 70], OperandSize::Dword)
}

#[test]
fn neg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 218], OperandSize::Qword)
}

#[test]
fn neg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(RDI, Four, 799834652, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 28, 189, 28, 130, 172, 47], OperandSize::Qword)
}

#[test]
fn neg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 217], OperandSize::Qword)
}

#[test]
fn neg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1588852038, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 156, 136, 70, 245, 179, 94], OperandSize::Qword)
}

#[test]
fn neg_9() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 218], OperandSize::Word)
}

#[test]
fn neg_10() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 178, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 153, 178, 0], OperandSize::Word)
}

#[test]
fn neg_11() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 223], OperandSize::Dword)
}

#[test]
fn neg_12() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectDisplaced(ECX, 49302535, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 153, 7, 76, 240, 2], OperandSize::Dword)
}

#[test]
fn neg_13() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 223], OperandSize::Qword)
}

#[test]
fn neg_14() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 28, 242], OperandSize::Qword)
}

#[test]
fn neg_15() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 217], OperandSize::Word)
}

#[test]
fn neg_16() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 51, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 88, 51], OperandSize::Word)
}

#[test]
fn neg_17() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 222], OperandSize::Dword)
}

#[test]
fn neg_18() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1286395698, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 156, 206, 50, 215, 172, 76], OperandSize::Dword)
}

#[test]
fn neg_19() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 219], OperandSize::Qword)
}

#[test]
fn neg_20() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 28, 112], OperandSize::Qword)
}

#[test]
fn neg_21() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 220], OperandSize::Qword)
}

#[test]
fn neg_22() {
    run_test(&Instruction { mnemonic: Mnemonic::NEG, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1497462625, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 28, 69, 97, 119, 65, 89], OperandSize::Qword)
}

