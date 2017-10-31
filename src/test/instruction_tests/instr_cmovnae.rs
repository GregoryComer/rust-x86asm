use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 206], OperandSize::Word)
}

#[test]
fn cmovnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 23861, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 138, 53, 93], OperandSize::Word)
}

#[test]
fn cmovnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 242], OperandSize::Dword)
}

#[test]
fn cmovnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(DI)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 58], OperandSize::Dword)
}

#[test]
fn cmovnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 213], OperandSize::Qword)
}

#[test]
fn cmovnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(RBX, 1144066455, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 171, 151, 17, 49, 68], OperandSize::Qword)
}

#[test]
fn cmovnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 212], OperandSize::Word)
}

#[test]
fn cmovnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BX, 3011, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 191, 195, 11], OperandSize::Word)
}

#[test]
fn cmovnae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 251], OperandSize::Dword)
}

#[test]
fn cmovnae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 20, 193], OperandSize::Dword)
}

#[test]
fn cmovnae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 238], OperandSize::Qword)
}

#[test]
fn cmovnae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RSI, 1721742489, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 166, 153, 180, 159, 102], OperandSize::Qword)
}

#[test]
fn cmovnae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 244], OperandSize::Qword)
}

#[test]
fn cmovnae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 28, 155], OperandSize::Qword)
}

