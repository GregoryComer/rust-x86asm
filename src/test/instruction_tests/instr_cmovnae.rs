use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 223], OperandSize::Word)
}

#[test]
fn cmovnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(SI, 18079, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 148, 159, 70], OperandSize::Word)
}

#[test]
fn cmovnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 231], OperandSize::Dword)
}

#[test]
fn cmovnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 261737864, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 52, 253, 136, 205, 153, 15], OperandSize::Dword)
}

#[test]
fn cmovnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 233], OperandSize::Qword)
}

#[test]
fn cmovnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1340973353, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 60, 117, 41, 161, 237, 79], OperandSize::Qword)
}

#[test]
fn cmovnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 246], OperandSize::Word)
}

#[test]
fn cmovnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(DI, 5709, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 173, 77, 22], OperandSize::Word)
}

#[test]
fn cmovnae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 239], OperandSize::Dword)
}

#[test]
fn cmovnae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(ECX, 418780240, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 185, 80, 20, 246, 24], OperandSize::Dword)
}

#[test]
fn cmovnae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 214], OperandSize::Qword)
}

#[test]
fn cmovnae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 46], OperandSize::Qword)
}

#[test]
fn cmovnae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 227], OperandSize::Qword)
}

#[test]
fn cmovnae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 40], OperandSize::Qword)
}

