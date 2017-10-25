use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 202], OperandSize::Word)
}

#[test]
fn cmovp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BP)), operand2: Some(Indirect(SI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 44], OperandSize::Word)
}

#[test]
fn cmovp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 243], OperandSize::Dword)
}

#[test]
fn cmovp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SI)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 50], OperandSize::Dword)
}

#[test]
fn cmovp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 242], OperandSize::Qword)
}

#[test]
fn cmovp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 28, 195], OperandSize::Qword)
}

#[test]
fn cmovp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 221], OperandSize::Word)
}

#[test]
fn cmovp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ECX)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 13], OperandSize::Word)
}

#[test]
fn cmovp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 230], OperandSize::Dword)
}

#[test]
fn cmovp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 28, 219], OperandSize::Dword)
}

#[test]
fn cmovp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 233], OperandSize::Qword)
}

#[test]
fn cmovp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 18], OperandSize::Qword)
}

#[test]
fn cmovp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 251], OperandSize::Qword)
}

#[test]
fn cmovp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 774359205, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 20, 125, 165, 200, 39, 46], OperandSize::Qword)
}

