use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovo_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 219], OperandSize::Word)
}

#[test]
fn cmovo_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(CX)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 13], OperandSize::Word)
}

#[test]
fn cmovo_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 233], OperandSize::Dword)
}

#[test]
fn cmovo_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(CX)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 9], OperandSize::Dword)
}

#[test]
fn cmovo_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 250], OperandSize::Qword)
}

#[test]
fn cmovo_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 41], OperandSize::Qword)
}

#[test]
fn cmovo_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 223], OperandSize::Word)
}

#[test]
fn cmovo_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBP)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 44], OperandSize::Word)
}

#[test]
fn cmovo_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 254], OperandSize::Dword)
}

#[test]
fn cmovo_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1805666384, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 188, 66, 80, 72, 160, 107], OperandSize::Dword)
}

#[test]
fn cmovo_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 237], OperandSize::Qword)
}

#[test]
fn cmovo_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 16], OperandSize::Qword)
}

#[test]
fn cmovo_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 251], OperandSize::Qword)
}

#[test]
fn cmovo_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 794432665, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 12, 93, 153, 20, 90, 47], OperandSize::Qword)
}

