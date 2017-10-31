use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 254], OperandSize::Word)
}

#[test]
fn bsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SP)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 36], OperandSize::Word)
}

#[test]
fn bsr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 230], OperandSize::Dword)
}

#[test]
fn bsr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EAX, 270948994, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 184, 130, 90, 38, 16], OperandSize::Dword)
}

#[test]
fn bsr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 202], OperandSize::Qword)
}

#[test]
fn bsr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 659700696, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 60, 93, 216, 59, 82, 39], OperandSize::Qword)
}

#[test]
fn bsr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 245], OperandSize::Word)
}

#[test]
fn bsr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EBP)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 189, 44], OperandSize::Word)
}

#[test]
fn bsr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 209], OperandSize::Dword)
}

#[test]
fn bsr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 60, 79], OperandSize::Dword)
}

#[test]
fn bsr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 233], OperandSize::Qword)
}

#[test]
fn bsr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RAX, 914236232, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 189, 144, 72, 35, 126, 54], OperandSize::Qword)
}

#[test]
fn bsr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 250], OperandSize::Qword)
}

#[test]
fn bsr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BSR, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 2144208887, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 189, 140, 120, 247, 7, 206, 127], OperandSize::Qword)
}

