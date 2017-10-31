use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 255], OperandSize::Word)
}

#[test]
fn cmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 48], OperandSize::Word)
}

#[test]
fn cmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 205], OperandSize::Dword)
}

#[test]
fn cmovb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1549614501, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 36, 213, 165, 61, 93, 92], OperandSize::Dword)
}

#[test]
fn cmovb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 207], OperandSize::Qword)
}

#[test]
fn cmovb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(CX)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 11], OperandSize::Qword)
}

#[test]
fn cmovb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 253], OperandSize::Word)
}

#[test]
fn cmovb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(BP, 208, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 166, 208, 0], OperandSize::Word)
}

#[test]
fn cmovb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 252], OperandSize::Dword)
}

#[test]
fn cmovb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1091483979, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 140, 150, 75, 185, 14, 65], OperandSize::Dword)
}

#[test]
fn cmovb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 252], OperandSize::Qword)
}

#[test]
fn cmovb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 844794623, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 188, 127, 255, 138, 90, 50], OperandSize::Qword)
}

#[test]
fn cmovb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 251], OperandSize::Qword)
}

#[test]
fn cmovb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVB, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RDX, 757551865, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 138, 249, 82, 39, 45], OperandSize::Qword)
}

