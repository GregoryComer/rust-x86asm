use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 215], OperandSize::Word)
}

#[test]
fn cmovna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(SI, 4747, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 188, 139, 18], OperandSize::Word)
}

#[test]
fn cmovna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 236], OperandSize::Dword)
}

#[test]
fn cmovna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1055508469, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 172, 194, 245, 199, 233, 62], OperandSize::Dword)
}

#[test]
fn cmovna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 202], OperandSize::Qword)
}

#[test]
fn cmovna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 748320950, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 180, 152, 182, 120, 154, 44], OperandSize::Qword)
}

#[test]
fn cmovna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 218], OperandSize::Word)
}

#[test]
fn cmovna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 8], OperandSize::Word)
}

#[test]
fn cmovna_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 243], OperandSize::Dword)
}

#[test]
fn cmovna_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 757398495, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 164, 216, 223, 251, 36, 45], OperandSize::Dword)
}

#[test]
fn cmovna_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 234], OperandSize::Qword)
}

#[test]
fn cmovna_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1320253711, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 188, 150, 15, 121, 177, 78], OperandSize::Qword)
}

#[test]
fn cmovna_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 234], OperandSize::Qword)
}

#[test]
fn cmovna_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 20, 159], OperandSize::Qword)
}

