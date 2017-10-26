use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 212], OperandSize::Word)
}

#[test]
fn cmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 134, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 154, 134, 0], OperandSize::Word)
}

#[test]
fn cmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 246], OperandSize::Dword)
}

#[test]
fn cmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(BP)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 42], OperandSize::Dword)
}

#[test]
fn cmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 203], OperandSize::Qword)
}

#[test]
fn cmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 36, 88], OperandSize::Qword)
}

#[test]
fn cmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 207], OperandSize::Word)
}

#[test]
fn cmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(BX, 17003, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 159, 107, 66], OperandSize::Word)
}

#[test]
fn cmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 207], OperandSize::Dword)
}

#[test]
fn cmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 44, 219], OperandSize::Dword)
}

#[test]
fn cmovb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 250], OperandSize::Qword)
}

#[test]
fn cmovb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1831709643, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 156, 71, 203, 171, 45, 109], OperandSize::Qword)
}

#[test]
fn cmovb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 218], OperandSize::Qword)
}

#[test]
fn cmovb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1706578164, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 12, 125, 244, 80, 184, 101], OperandSize::Qword)
}

