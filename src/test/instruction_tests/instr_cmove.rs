use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmove_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 228], OperandSize::Word)
}

#[test]
fn cmove_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(BX, 8494, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 159, 46, 33], OperandSize::Word)
}

#[test]
fn cmove_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 217], OperandSize::Dword)
}

#[test]
fn cmove_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(DI)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 56], OperandSize::Dword)
}

#[test]
fn cmove_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 229], OperandSize::Qword)
}

#[test]
fn cmove_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1285204890, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 36, 213, 154, 171, 154, 76], OperandSize::Qword)
}

#[test]
fn cmove_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 229], OperandSize::Word)
}

#[test]
fn cmove_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 19, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 68, 72, 19], OperandSize::Word)
}

#[test]
fn cmove_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 212], OperandSize::Dword)
}

#[test]
fn cmove_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EBX, 825008420, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 179, 36, 161, 44, 49], OperandSize::Dword)
}

#[test]
fn cmove_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 247], OperandSize::Qword)
}

#[test]
fn cmove_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 68, 28, 185], OperandSize::Qword)
}

#[test]
fn cmove_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 211], OperandSize::Qword)
}

#[test]
fn cmove_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVE, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RAX, 516347037, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 68, 144, 157, 212, 198, 30], OperandSize::Qword)
}

