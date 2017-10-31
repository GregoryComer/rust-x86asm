use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 212], OperandSize::Word)
}

#[test]
fn cmovc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 26729, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 154, 105, 104], OperandSize::Word)
}

#[test]
fn cmovc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 245], OperandSize::Dword)
}

#[test]
fn cmovc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(EDI, 431607399, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 191, 103, 206, 185, 25], OperandSize::Dword)
}

#[test]
fn cmovc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 218], OperandSize::Qword)
}

#[test]
fn cmovc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(SI)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 51], OperandSize::Qword)
}

#[test]
fn cmovc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 247], OperandSize::Word)
}

#[test]
fn cmovc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 25], OperandSize::Word)
}

#[test]
fn cmovc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 223], OperandSize::Dword)
}

#[test]
fn cmovc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1113469193, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 20, 245, 9, 49, 94, 66], OperandSize::Dword)
}

#[test]
fn cmovc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 245], OperandSize::Qword)
}

#[test]
fn cmovc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 43], OperandSize::Qword)
}

#[test]
fn cmovc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 233], OperandSize::Qword)
}

#[test]
fn cmovc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 60, 208], OperandSize::Qword)
}

