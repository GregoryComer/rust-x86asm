use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 251], OperandSize::Word)
}

#[test]
fn cmovp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 1396, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 152, 116, 5], OperandSize::Word)
}

#[test]
fn cmovp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 238], OperandSize::Dword)
}

#[test]
fn cmovp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SI)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 49], OperandSize::Dword)
}

#[test]
fn cmovp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 228], OperandSize::Qword)
}

#[test]
fn cmovp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(CX)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 15], OperandSize::Qword)
}

#[test]
fn cmovp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 210], OperandSize::Word)
}

#[test]
fn cmovp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 82, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 114, 82], OperandSize::Word)
}

#[test]
fn cmovp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 217], OperandSize::Dword)
}

#[test]
fn cmovp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 60, 208], OperandSize::Dword)
}

#[test]
fn cmovp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 207], OperandSize::Qword)
}

#[test]
fn cmovp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1622495575, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 140, 94, 87, 81, 181, 96], OperandSize::Qword)
}

#[test]
fn cmovp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 212], OperandSize::Qword)
}

#[test]
fn cmovp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 54], OperandSize::Qword)
}

